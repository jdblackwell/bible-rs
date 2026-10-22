<!-- A topic's page (+page.server.ts): its passages, each linked to its place in the reader. -->
<script lang="ts">
	import Frame from '#lib/read/seo/Frame.svelte';
	import Meta from '#lib/read/seo/Meta.svelte';
	import Runs from '#lib/read/seo/Runs.svelte';
	import type { PageProps } from './$types';

	let { data }: PageProps = $props();
</script>

<Meta {...data.meta} />

<Frame crumbs={data.meta.crumbs} reader={{ href: '/read', label: 'Open the reader' }} notice={data.info.copyright}>
	<header class="head">
		<h1>{data.topic.title}</h1>
		<p class="description">{data.topic.description}</p>
	</header>
	{#each data.passages as p (p.label)}
		<section class="passage">
			<h2><a href={p.href}>{p.label}</a> <span class="abbrev">{data.info.abbrev}</span></h2>
			<p>
				{#each p.verses as v (v.number)}{#if p.verses.length > 1}<span class="vn">{v.number}</span>{/if}<Runs
						runs={v.runs}
					/>{' '}{/each}
			</p>
		</section>
	{/each}
</Frame>

<style>
	.head {
		margin-bottom: 40px;
	}

	h1 {
		font: var(--type-display);
		letter-spacing: var(--type-display-tracking);
		font-variation-settings: 'opsz' 48;
	}

	.description {
		margin-top: 12px;
		color: var(--ink-2);
		font: var(--type-subtitle);
	}

	.passage {
		padding: 16px 0;
		border-bottom: 1px solid var(--separator);
	}

	h2 {
		margin-bottom: 6px;
		font: var(--type-ui-strong);
	}

	h2 a {
		color: var(--accent);
	}

	.abbrev {
		color: var(--ink-3);
		font: var(--type-small);
	}

	.passage p {
		font: var(--type-snippet);
		font-size: 18px;
		line-height: 30px;
	}

	.vn {
		margin-right: 3px;
		color: var(--verse);
		font: 600 11px/1 var(--font-ui);
		vertical-align: top;
		position: relative;
		top: 4px;
	}
</style>
