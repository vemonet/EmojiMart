import data from "@emoji-mart/data";
import { Picker } from 'emoji-mart'
import { clipboard } from "@tauri-apps/api";
import { appWindow } from '@tauri-apps/api/window'
import { invoke } from '@tauri-apps/api/tauri'
import { onCleanup, onMount } from 'solid-js';

export interface EmojiData {
  // Only some properties, waiting for https://github.com/missive/emoji-mart/pull/789
  id: string;
  name: string;
  colons: string;
  native: string;
}

function App(): any {

  // Add to clipboard and close when clicking an emoji
  const onEmojiSelect = (emoji: EmojiData) => {
    clipboard.writeText(emoji.native);
    appWindow.hide()
    invoke('trigger_paste')
  }

  // Close when hit <Esc>
  const handleKeypress = (event: any) => {
    if (event.code === "Escape") {
      appWindow.close()
    }
    // TODO: else if key somewhere from A to Z, bring back focus on the picker search input
  }

  onMount(() => {
    document.addEventListener('keypress', handleKeypress);
  })

  onCleanup(() => {
    document.removeEventListener('keypress', handleKeypress);
  })

  return new Picker({ data, onEmojiSelect, autoFocus: true, dynamicWidth: true})
}

export default App;
