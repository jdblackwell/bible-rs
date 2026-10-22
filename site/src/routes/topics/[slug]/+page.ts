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
		const ref = parseReference(reference);
		// A typo in the file stops the build rather than printing an empty passage.
		if (!ref) throw new Error(`topics.json: "${reference}" (${topic.slug}) isn't a reference`);
		const last = ref.verseEnd ?? ref.verse;
		const verses = c
			.verses(TRANSLATION, ref.chapterIndex)
			.filter((v) => ref.verse === null || (v.number >= ref.verse && v.number <= last!));
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
