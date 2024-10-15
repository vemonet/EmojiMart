<script lang="ts">
	import { Picker } from 'emoji-mart'
	import { writeText } from '@tauri-apps/plugin-clipboard-manager'
	import { getCurrentWindow } from '@tauri-apps/api/window'
	import { invoke } from '@tauri-apps/api/core'
	import { getMatches } from '@tauri-apps/plugin-cli'
	import { locale } from '@tauri-apps/plugin-os'
	import { onMount, onDestroy } from 'svelte'
	import type { EmojiData } from './+page'
	// import data from '@emoji-mart/data'

	const acceptedThemes = ['auto', 'light', 'dark']
	let theme = 'auto'
	let lang = 'en'
	// Supported: ar, be, cs, de, en, es, fa, fi, fr, hi, it, ja, ko, nl, pl, pt, ru, sa, tr, uk, vi, zh
	let i18n: any = null
	let data: any = null
	let picker: any
	let keep = false
	let pickMulti = false
	let selection: string[] = []

	// Add to clipboard and close when clicking an emoji
	const onEmojiSelect = async (emoji: EmojiData) => {
		if (pickMulti) {
			selection.push(emoji.native)
		} else {
			// Adding to the clipboard and hiding the window is done here, Rust handles auto-paste and exit
			if (!keep) await writeText(emoji.native)
			await getCurrentWindow().hide()
			await invoke('trigger_paste', { emoji: emoji.native })
		}
	}

	const handleKeypress = (event: any) => {
		// Close when hit <Esc>
		if (event.code === 'Escape') {
			getCurrentWindow().close()
		}
		// TODO: pick multiple emojis when shift selected
		if (event.code === 'Shift' || event.shiftKey) {
			pickMulti = true
		}
	}

	onMount(async () => {
		// Get language from system
		lang = (await locale())?.slice(0, 2) || lang
		// Get arguments
		let matches = await getMatches()
		if (matches.args['keep'].value?.toString().toLowerCase() === 'true') keep = true
		if (matches.args['lang'].value) lang = matches.args['lang'].value?.toString().toLowerCase()
		if (matches.args['theme'].value) theme = matches.args['theme'].value?.toString().toLowerCase()
		else {
			const sysTheme = await getCurrentWindow().theme()
			if (sysTheme) theme = sysTheme
		}
		if (acceptedThemes.indexOf(theme) === -1) {
			console.error(
				`The theme provided "${theme}" is not valid, we will use auto by default. Please use one of: light, dark, auto.`
			)
			theme = 'auto'
		}

		// Load the emoji data from this repo, precompiled for different languages
		try {
			i18n = (await import(`@emoji-mart/data/i18n/${lang}.json`)).default
			data = (await import(`../data/${lang}.json`)).default
		} catch (err) {
			console.error(`Language ${lang} not supported, loading default`)
		}

		// Create the picker
		picker.append(
			new Picker({
				data,
				onEmojiSelect,
				theme,
				i18n,
				autoFocus: true,
				dynamicWidth: true,
				locale: lang
			})
		)
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
