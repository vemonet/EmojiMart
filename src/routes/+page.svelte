<script lang="ts">
	import data from '@emoji-mart/data'
	import { Picker } from 'emoji-mart'
	import { clipboard } from '@tauri-apps/api'
	import { appWindow } from '@tauri-apps/api/window'
	import { invoke } from '@tauri-apps/api/tauri'
	import { getMatches } from '@tauri-apps/api/cli'
	import { onMount, onDestroy } from 'svelte'
	import type { EmojiData } from './+page'

	const acceptedThemes = ['auto', 'light', 'dark']
	let theme = 'auto'
	let lang = 'fr'
	let keep = false
	let picker: any
	let pickMulti = false
	let selection: string[] = []

	// Add to clipboard and close when clicking an emoji
	const onEmojiSelect = async (emoji: EmojiData) => {
		if (pickMulti) {
			selection.push(emoji.native)
		} else {
			const previous = await clipboard.readText()
			clipboard.writeText(emoji.native)
			appWindow.hide()
			if (previous) await invoke('trigger_paste', { emoji: emoji.native, keep, previous: previous })
			else await invoke('trigger_paste', { emoji: emoji.native, keep: false })
			if (keep && previous) clipboard.writeText(previous)
		}
	}

	const handleKeypress = (event: any) => {
		// Close when hit <Esc>
		if (event.code === 'Escape') {
			appWindow.close()
		}
		if (event.code === 'Shift' || event.shiftKey) {
			pickMulti = true
		}
	}

	onMount(async () => {
		// Get arguments
		let matches = await getMatches()
		if (matches.args['keep'].value?.toString().toLowerCase() === 'true') keep = true
		if (matches.args['lang'].value) lang = matches.args['lang'].value?.toString().toLowerCase()
		if (matches.args['theme'].value) theme = matches.args['theme'].value?.toString().toLowerCase()
		else {
			const sysTheme = await appWindow.theme()
			if (sysTheme) theme = sysTheme
		}
		if (acceptedThemes.indexOf(theme) === -1) {
			console.error(
				`The theme provided "${theme}" is not valid, we will use auto by default. Please use one of: light, dark, auto.`
			)
			theme = 'auto'
		}

		const i18n = (await import('@emoji-mart/data/i18n/fr.json')).default;

		// const i18n = (await import(`@emoji-mart/data/i18n/${lang}.json`)).default;

		// const langData = (await import(`@emoji-mart/data/i18n/data-${lang}.json`)).default;
		const langData = (await import(`../data/${lang}.json`)).default;
		// <Picker data={langData} locale=lang />

		// if (lang !== 'en') {
		// 	const i18n = (await import(`@emoji-mart/data/i18n/${lang}.json`)).default;
		// }

		// Create the picker
		picker.append(new Picker({ data: langData, onEmojiSelect, theme, i18n, autoFocus: true, dynamicWidth: true, locale: lang }))
		document.addEventListener('keypress', handleKeypress)
	})

	onDestroy(() => {
		document.removeEventListener('keypress', handleKeypress)
	})
</script>

<!-- https://svelte.dev/tutorial/bind-this -->
<section>
	<div bind:this={picker} />
	{#if selection.length > 0}
		<p>{selection.join(' ')}</p>
	{/if}
</section>
