import "./App.css";
import data from "@emoji-mart/data";
import { Picker } from 'emoji-mart'
import { clipboard, window } from "@tauri-apps/api";
// import { useKeyDownEvent } from "@solid-primitives/keyboard";

export interface EmojiData {
  // only some properties, waiting for https://github.com/missive/emoji-mart/pull/789
  // alternatively we could use the old types of @types/emoji-mart@3.0.9
  id: string;
  name: string;
  colons: string;
  native: string;
}

function App() {
  const onEmojiSelect = (emoji: EmojiData) => {
    clipboard.writeText(emoji.native);
    setTimeout(() => window.appWindow.close(), 0);
  };

  // TODO: close window when click outside or click ESC?
  // const event = useKeyDownEvent();
  // const e = event();
  // if (e) {
  //   console.log(e.key); // => "Q" | "ALT" | ... or null
  //   if (e.key === "ALT") {
  //     window.appWindow.close()
  //   }
  //   e.preventDefault(); // prevent default behavior or last keydown event
  // }

  return new Picker({ data, onEmojiSelect, autoFocus: true, dynamicWidth: true})

}

export default App;
