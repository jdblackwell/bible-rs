/**
 * Topical pages (/topics/<slug>, "Verses about hope"), written by hand in topics.json: each a
 * title, a description and the passages it gathers. The route (src/routes/topics/[slug]) prints
 * each passage in the KJV with a link to it, and the topics sitemap lists the pages. With no
 * topics in the file there are no pages, and the sitemap is empty.
 */
import data from './topics.json';

export interface Topic {
	/** The address, /topics/<slug>: lower case, digits and hyphens. */
	slug: string;
	/** The heading, and the page's title before " | Bible". */
	title: string;
	/** The search result's description, also shown under the heading. */
	description: string;
	/** References as the reader's jump field takes them ("John 3:16", "Ps 23:1-4", "Romans 8"), in order. */
	passages: string[];
}

export const TOPICS: Topic[] = data;

export const topicPath = (slug: string) => `/topics/${slug}`;
