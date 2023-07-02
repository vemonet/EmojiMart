import data from '@emoji-mart/data'
import {Picker} from 'emoji-mart'
import {clipboard} from '@tauri-apps/api'
import {appWindow} from '@tauri-apps/api/window'
import {invoke} from '@tauri-apps/api/tauri'
import {getMatches} from '@tauri-apps/api/cli'
import {onCleanup, onMount} from 'solid-js'

export interface EmojiData {
  // Only some properties, waiting for https://github.com/missive/emoji-mart/pull/789
  id: string
  name: string
  colons: string
  native: string
}
const acceptedThemes = ['light', 'dark', 'auto']

let theme = 'auto'
let keep = false

// Get args: keep previous clipboard item, and theme
getMatches().then((matches) => {
  if (matches.args['keep'].value?.toString().toLowerCase() === "true") keep = true
  if (matches.args['theme'].value) theme = matches.args['theme'].value?.toString().toLowerCase()
  else {
    // If no theme in args, get from systeme
    appWindow.theme().then((sysTheme) => {
      if (sysTheme && acceptedThemes.indexOf(sysTheme.toLowerCase()) !== -1) {
        theme = sysTheme.toLowerCase()
      }
    })
  }
})

function App(): any {

  // Add to clipboard and close when clicking an emoji
  const onEmojiSelect = (emoji: EmojiData) => {
    // clipboard.writeText(emoji.native)
    // appWindow.hide()
    // invoke('trigger_paste', {emoji: emoji.native, keep: false})

    // If nothing in the clipboard we copy the emoji anyway
    clipboard.readText()
      .then((previous) => {
        console.log("EMOJI keep", emoji.native, keep)
        clipboard.writeText(emoji.native)
        appWindow.hide()
        if (previous) invoke('trigger_paste', { emoji: emoji.native, previous: previous, keep: keep })
        else invoke('trigger_paste', {emoji: emoji.native, keep: false})
      })
      .catch(() => {
        clipboard.writeText(emoji.native)
        invoke('trigger_paste', { emoji: emoji.native, keep: false })
      })
  }

  // Close when hit <Esc>
  const handleKeypress = (event: any) => {
    if (event.code === 'Escape') {
      appWindow.close()
    }
    // TODO: else if key somewhere from A to Z, bring back focus on the picker search input?
  }

  onMount( async () => {
    document.addEventListener('keypress', handleKeypress)
  })

  onCleanup(() => {
    document.removeEventListener('keypress', handleKeypress)
  })

  return new Picker({data, onEmojiSelect, theme: theme, autoFocus: true, dynamicWidth: true})
}

export default App
