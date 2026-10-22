<script lang="ts">
	import type { Book } from '../core';
	import { follow } from '../ui/follow';

	interface Props {
		book: Book;
		/** The chapter being read (a chapter index), lit when it's in this book. */
		current: number;
		/** Chapters (indices) with highlights or notes. */
		activity: Set<number>;
		/** A chapter's address (each chip is a link to it). */
		href: (chapter: number) => string;
		onpick: (chapter: number) => void;
	}

	let { book, current, activity, href, onpick }: Props = $props();
</script>

<!-- The open book's chapters: 24 pt chips in sevens, the current one solid accent; a dot marks
     chapters with highlights or notes. -->
<div class="grid" role="group" aria-label="{book.name} chapters">
	{#each { length: book.chapters }, i (i)}
		{@const index = book.firstChapter + i}
		{@const here = index === current}
		<a
			href={href(index)}
			class={['chip', 'follow', { here, active: !here && activity.has(index) }]}
			aria-current={here || undefined}
			aria-label="{book.name} {i + 1}"
			use:follow={() => onpick(index)}
		>
			{i + 1}
		</a>
	{/each}
</div>

<style>
	.grid {
		display: grid;
		grid-template-columns: repeat(7, 24px);
		gap: 4px 8px;
		padding: 8px 0 8px 24px;
	}

	.chip {
		position: relative;
		display: grid;
		place-items: center;
		width: 24px;
		height: 24px;
		border-radius: 50%;
		color: var(--ink-2);
		font: var(--type-small);
		font-variant-numeric: tabular-nums;
		transition: background-color 100ms;
	}

	.chip:hover {
		background: var(--fill);
	}

	.chip:active {
		background: var(--fill-strong);
	}

	.chip.here {
		background: var(--accent);
		color: var(--on-accent);
		font-weight: 600;
	}

	.chip.active::after {
		content: '';
		position: absolute;
		left: 50%;
		top: 23px;
		width: 4px;
		height: 4px;
		margin-left: -2px;
		border-radius: 50%;
		background: var(--accent);
	}
</style>
