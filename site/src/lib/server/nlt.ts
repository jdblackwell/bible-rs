/**
 * The rules /api/nlt keeps (src/routes/api/nlt/+server.ts). Tyndale's permission terms let
 * non-commercial apps quote the NLT up to 500 verses, never a complete book, with their credit
 * line. So the proxy answers one chapter or less a request, refuses whole books and passages that
 * cross chapters, and a reader's browser can ask for a limited number of chapters a minute and a
 * day.
 */
import { BOOKS, bookSlug } from '../read/books';
import type { Book } from '../read/core';

/** The most verses one request to the API may ask for (Tyndale's limit with a key). */
export const MAX_VERSES = 500;

/** Psalm 119's 176 verses are the most a chapter has, so every chapter fits one request. */
export const MAX_VERSE = 176;

/** A passage in one chapter: the whole chapter, or verses `[first, last]` of it. */
export interface Passage {
	book: Book;
	chapter: number;
	verses: [number, number] | null;
}

export type Parsed = { ok: true; passage: Passage } | { ok: false; error: string };

/** Book names as typed: names, URL slugs, abbreviations, with or without spaces. */
const NAMES = new Map<string, Book>();
for (const book of BOOKS) {
	const osis = book.abbrev.replace(/ /g, '');
	for (const name of [book.name, bookSlug(book), book.abbrev, osis, apiBook(book)])
		NAMES.set(name.toLowerCase().replace(/[\s-]+/g, ' '), book);
}
NAMES.set('psalm', BOOKS[18]);

const fail = (error: string): Parsed => ({ ok: false, error });

/**
 * Reads a reference ("John 3", "John 3:16-18", "1 John 4", "John.3.16") as a passage the proxy
 * may fetch: one chapter or part of one.
 */
export function parsePassage(input: string | null): Parsed {
	const ref = (input ?? '').trim().replace(/\s+/g, ' ');
	if (!ref) return fail('Ask for a passage, like ?ref=John+3');
	if (ref.length > 60) return fail('That reference is too long');
	if (/[;,&]/.test(ref)) return fail('One passage at a time');
	const m = /^((?:[1-3][ -]?)?[a-z]+(?:[ -][a-z]+)*)\.?(?: ?(.*))?$/i.exec(ref);
	const book = m && NAMES.get(m[1].toLowerCase().replace(/[\s-]+/g, ' '));
	if (!m || !book) return fail(`No book matches “${ref}”`);
	const rest = (m[2] ?? '').replace(/\s/g, '');
	if (!rest) return fail(`That’s all of ${book.name}: the NLT comes one chapter or less at a time`);
	const range = /^(\d+)(?:[:.](\d+)(?:[-–](\d+))?)?$/.exec(rest);
	if (!range) {
		return /^\d+(?:[:.]\d+)?[-–]\d+(?:[:.]\d+)?$/.test(rest)
			? fail('That runs across chapters: the NLT comes one chapter or less at a time')
			: fail(`Couldn’t read “${ref}”`);
	}
	const chapter = Number(range[1]);
	if (chapter < 1 || chapter > book.chapters)
		return fail(`${book.name} has ${book.chapters} chapter${book.chapters === 1 ? '' : 's'}`);
	if (!range[2]) return { ok: true, passage: { book, chapter, verses: null } };
	const first = Number(range[2]);
	const last = range[3] ? Number(range[3]) : first;
	if (first < 1 || last < first || last > MAX_VERSE) return fail(`No such verses in ${book.name} ${chapter}`);
	return { ok: true, passage: { book, chapter, verses: [first, last] } };
}

/** The API's name for a book: OSIS, except five it spells its own way (online/nlt.rs `api_book`). */
export function apiBook(book: Book): string {
	const osis = book.abbrev.replace(/ /g, '');
	return { '1Thess': '1Thes', '2Thess': '2Thes', '1John': '1Jn', '2John': '2Jn', '3John': '3Jn' }[osis] ?? osis;
}

/** The passage as the API takes it: "John.3", "John.3.16-18". */
export function apiReference(p: Passage): string {
	const [first, last] = p.verses ?? [0, 0];
	const verses = !p.verses ? '' : last > first ? `.${first}-${last}` : `.${first}`;
	return `${apiBook(p.book)}.${p.chapter}${verses}`;
}

/** "John 3", "John 3:16–18". */
export function passageLabel(p: Passage): string {
	const name = p.book.name === 'Psalms' ? 'Psalm' : p.book.name;
	if (!p.verses) return `${name} ${p.chapter}`;
	const [first, last] = p.verses;
	return `${name} ${p.chapter}:${first}${last > first ? `–${last}` : ''}`;
}

/** The whole of a one-chapter book (Jude 1): shown, but never kept, even at the edge. */
export const wholeBook = (p: Passage) => p.book.chapters === 1 && !p.verses;

/**
 * The verses of the API's page and their count: from the first `<verse_export>` to the end of the
 * last, without the page's head and the passage's title. Null when no verse came back.
 */
export function verseExcerpt(html: string): { html: string; verses: number } | null {
	const start = html.indexOf('<verse_export');
	const end = html.lastIndexOf('</verse_export>');
	if (start < 0 || end < start) return null;
	const excerpt = html.slice(start, end + '</verse_export>'.length);
	return { html: excerpt, verses: excerpt.match(/<verse_export[\s>]/g)?.length ?? 0 };
}

export interface Limit {
	max: number;
	/** The window, in ms. */
	per: number;
}

/**
 * Requests per key (a client's address) in fixed windows, each limit counted separately. It lives
 * in one function instance, so it slows a single client down rather than capping the site;
 * Tyndale's own daily limit on the key is the backstop.
 */
export class RateLimiter {
	readonly #windows = new Map<string, { start: number; count: number }[]>();

	constructor(
		readonly limits: Limit[],
		/** Past this many clients, ones whose windows have all ended are forgotten (then the oldest). */
		readonly maxKeys = 50_000
	) {}

	/** Counts a request; returns 0 when it's allowed, else the seconds until it would be. */
	take(key: string, now = Date.now()): number {
		let windows = this.#windows.get(key);
		if (!windows) {
			if (this.#windows.size >= this.maxKeys) this.#prune(now);
			windows = this.limits.map(() => ({ start: now, count: 0 }));
			this.#windows.set(key, windows);
		}
		let wait = 0;
		this.limits.forEach((limit, i) => {
			const w = windows[i];
			if (now - w.start >= limit.per) windows[i] = { start: now, count: 0 };
			else if (w.count >= limit.max) wait = Math.max(wait, w.start + limit.per - now);
		});
		if (wait > 0) return Math.ceil(wait / 1000);
		for (const w of windows) w.count++;
		return 0;
	}

	get size(): number {
		return this.#windows.size;
	}

	#prune(now: number) {
		for (const [key, windows] of this.#windows) {
			if (this.limits.every((limit, i) => now - windows[i].start >= limit.per)) this.#windows.delete(key);
		}
		for (const key of this.#windows.keys()) {
			if (this.#windows.size < this.maxKeys) break;
			this.#windows.delete(key);
		}
	}
}

/** A reader turning pages: 20 chapters a minute, 300 a day, per address. */
export const LIMITS: Limit[] = [
	{ max: 20, per: 60_000 },
	{ max: 300, per: 86_400_000 }
];
