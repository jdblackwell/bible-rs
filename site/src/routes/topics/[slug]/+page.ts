import { error } from '@sveltejs/kit';
import { NAME } from '#lib/config.js';
import { chapterPath, versePath } from '#lib/read/books.js';
import { core, parseReference } from '#lib/read/core.js';
import { VERSE_PAGES } from '#lib/read/seo/meta.js';
import { TOPICS, topicPath } from '#lib/topics/index.js';
import type { EntryGenerator, PageLoad } from './$types';

/** A topic from lib/topics/topics.json, its passages in the KJV: a plain page, no app (see the verse page's +page.ts). */
export const prerender = true;
export const csr = false;

export const entries: EntryGenerator = () => TOPICS.map(({ slug }) => ({ slug }));

const TRANSLATION = VERSE_PAGES[0];

export const load: PageLoad = async ({ params }) => {
	const topic = TOPICS.find((t) => t.slug === params.slug);
	if (!topic) error(404, 'No such topic');
	const c = await core();
	const info = await c.load(TRANSLATION);
	const passages = topic.passages.map((reference) => {
		// A typo in the file stops the build rather than printing an empty or shortened passage.
		const wrong = (why: string) => new Error(`topics.json: "${reference}" (${topic.slug}) ${why}`);
		const ref = parseReference(reference);
		if (!ref) throw wrong(`isn't a reference`);
		const chapter = c.verses(TRANSLATION, ref.chapterIndex);
		const first = ref.verse ?? chapter[0]?.number;
		const last = ref.verseEnd ?? ref.verse ?? chapter.at(-1)?.number;
		const has = (n: number | undefined) => chapter.some((v) => v.number === n);
		if (!has(first) || !has(last) || last! < first!) throw wrong(`names verses the ${info.abbrev} doesn't have`);
		const verses = chapter.filter((v) => v.number >= first! && v.number <= last!);
		return {
			label: ref.label,
			href: ref.verse
				? versePath(c.books, TRANSLATION, ref.chapterIndex, ref.verse)
				: chapterPath(c.books, TRANSLATION, ref.chapterIndex),
			verses: verses.map((v) => ({ number: v.number, runs: v.runs }))
		};
	});
	const path = topicPath(topic.slug);
	return {
		meta: {
			title: `${topic.title} | ${NAME}`,
			description: topic.description,
			path,
			crumbs: [
				{ name: 'Home', path: '/' },
				{ name: topic.title, path }
			]
		},
		topic,
		info,
		passages
	};
};
