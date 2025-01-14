<script lang="ts">
  import Keyboard from "simple-keyboard";
  import "simple-keyboard/build/css/index.css";
  import "../app.css";
  import { onDestroy, onMount } from "svelte";
  import IconButton from "$lib/components/IconButton.svelte";
  import {
    getCurrentWindow,
    Window,
    LogicalSize,
    currentMonitor,
    PhysicalSize,
  } from "@tauri-apps/api/window";
  import NSpell from "nspell";
  import { invoke } from "@tauri-apps/api/core";
  import { fade, fly } from "svelte/transition";
  import { animateMeSize, animateMe } from "tauri-plugin-taurimation-api";
  import { listen, type UnlistenFn } from "@tauri-apps/api/event";
  let speller: NSpell | undefined = $state();
  let keyboard: HTMLDivElement | undefined = $state();
  let keyboardHost: Keyboard | undefined = $state();
  let window: Window | undefined = $state();
  let size = $state([new LogicalSize(600, 275), new LogicalSize(38, 38)]);
  let currentSize = $state(0);
  let activeWindows: UnlistenFn | undefined = $state();
  let lastKnownWindow: WindowInfo | undefined = $state();
  let vboardInfo: WindowInfo | undefined = $state();
  let monitorSize: PhysicalSize | undefined = $state();
  let isCtrlPressed = $state(false);
  type WindowInfo = {
    hwnd: number;
    title: string;
  };
  function setOpacity(on: number) {
    keyboard?.setAttribute("style", `opacity:${on == 0 ? 1 : 0}`);
  }
  function doAnimate(cur: number) {
    let from;
    let to;
    if (cur == 0) {
      from = [size[1].width, size[1].height];
      to = [size[0].width, size[0].height];
    } else {
      to = [size[1].width, size[1].height];
      from = [size[0].width, size[0].height];
    }

    animateMeSize("main", 50, 50, to, "EaseOutQuad", from);
  }
  $effect(() => {
    setOpacity(currentSize);
    doAnimate(currentSize);
    //window?.setSize(size[currentSize]);
  });
  $inspect(currentSize);

  function handleShift() {
    if (!keyboardHost) return;
    let currentLayout = keyboardHost.options.layoutName;
    let shiftToggle = currentLayout === "default" ? "shift" : "default";

    keyboardHost.setOptions({
      layoutName: shiftToggle,
    });
  }

  function handleControl() {
    isCtrlPressed = !isCtrlPressed;
    if (isCtrlPressed) keyboardHost?.addButtonTheme("{control}", "hg-red");
    else keyboardHost?.removeButtonTheme("{control}", "hg-red");
  }

  let currentWord = $state("");
  let suggestionWords: string[] = $state([]);
  function spellCheck(input: string) {
    invoke<string[]>("spell_check", { input: input, limit: 5 }).then(
      (result) => {
        suggestionWords = result;
      },
    );
  }
  onDestroy(async () => {
    if (activeWindows) activeWindows();
    if (isOnTimer) clearInterval(isOnTimer);
  });
  onMount(async () => {
    window = getCurrentWindow();

    activeWindows = await listen(
      "active-window",
      (result: { event: string; payload: WindowInfo[] }) => {
        if (result.payload.length > 1) {
          const pl = result.payload;
          const foreground = pl[1].title.replace(/\x00/g, "");
          const last = pl[0].title.replace(/\x00/g, "");
          if (foreground === "vboard") {
            vboardInfo = pl[1];
            lastKnownWindow = pl[0];
          }
        }
      },
    );
    if (keyboard)
      keyboardHost = new Keyboard(keyboard, {
        onKeyPress(button, e) {
          if (button === "{control}") handleControl();
          if (button === "{shift}" || button === "{lock}") handleShift();
          if (["{space}"].includes(button) || isCtrlPressed) {
            keyboardHost?.clearInput();
          }
          sendKey(button);
        },
        onChange(input, e) {
          const word = input.trim();
          currentWord = word;

          if (word.length > 0) {
            spellCheck(currentWord);
          } else {
            suggestionWords = [];
          }
        },
        mergeDisplay: true,
        useMouseEvents: true,
        theme: "hg-theme-default mtheme",
        layout: {
          default: [
            "{esc} ` 1 2 3 4 5 6 7 8 9 0 - = {bksp}",
            "{tab} q w e r t y u i o p [ ] \\",
            "{lock} a s d f g h j k l ; ' {enter}",
            "{shift} z x c v b n m , . / {shift}",
            "{control} {meta} {space}",
          ],
          shift: [
            "{esc} ~ ! @ # $ % ^ &amp; * ( ) _ + {bksp}",
            "{tab} Q W E R T Y U I O P { } |",
            '{lock} A S D F G H J K L : " {enter}',
            "{shift} Z X C V B N M &lt; &gt; ? {shift}",
            "{control} {space} {meta}",
          ],
        },
        buttonTheme: [
          {
            class: "hg-red",
            buttons: "{bksp}",
          },
          {
            class: "hg-red",
            buttons: "{esc}",
          },
          {
            class: "hg-red",
            buttons: "{enter}",
          },
        ],
        display: {
          "{enter}": "Enter ↩︎",
          "{bksp}": "⌫",
          "{shift}": "shift ⇧",
          "{meta}": "⌘",
          "{control}": "ctrl ⌃",
        },
      });
  });

  async function sendKey(key: string) {
    // if (lastKnownWindow)
    //   await invoke("focus_window", { hwnd: lastKnownWindow?.hwnd });
    switch (key) {
      case "{control}":
      case "{lock}":
      case "{shift}":
        break;
      case "{tab}":
      case "{meta}":
      case "{esc}":
      case "{space}":
      case "{enter}":
      case "{bksp}":
        invoke("send_key_alt", { key: key }).then((result) => {
          console.log(result);
        });
        break;

      default:
        invoke("send_key", { key: key,ctrl:isCtrlPressed }).then((result) => {
        });
        if(isCtrlPressed){
          handleControl();
        }
        break;
    }
  }
  let isOn = $state(false);
  $effect(() => {
    notFocus(isOn);
  });

  async function resetPosition() {
    monitorSize = (await currentMonitor())?.size;
    const GAP = 100;
    if (monitorSize)
      animateMe(
        "main",
        50,
        50,
        [monitorSize?.width - GAP, monitorSize?.height - GAP],
        "EaseInBounce",
      );
  }

  let isOnTimer: NodeJS.Timeout;
  function notFocus(on: boolean) {
    if (on) {
      if (isOnTimer) clearInterval(isOnTimer);
      return;
    }

    if (isOnTimer) clearInterval(isOnTimer);

    let counter = 0;
    isOnTimer = setInterval(() => {
      if (counter > 2) {
        keyboardHost?.clearInput();
        suggestionWords = [];
        currentWord = "";
        console.log("im out", counter);
      }
      if (counter > 10) {
        currentSize = 1;
        //resetPosition();
        clearInterval(isOnTimer);
      }

      counter++;
    }, 1000);
  }
</script>

<svelte:document
  onmouseenter={() => {
    isOn = true;
  }}
  onmouseleave={(e) => {
    isOn = false;
  }}
/>
<main class="container">
  <div data-tauri-drag-region class="panel">
    <IconButton
      icon="resize"
      onclick={() => {
        currentSize = (currentSize + 1) % 2;
      }}
    />

    <div class="suggestion">
      <span class="current-word">{currentWord}</span>
      {#each suggestionWords as suggestion, index}
        <button
          onclick={() => {
            invoke("send_key_replace", { key: suggestion });
          }}
          transition:fade={{ delay: 100 }}
          class="sugg">{suggestion}</button
        >
      {/each}
    </div>
    <IconButton
      icon="close"
      onclick={() => {
        window?.close();
      }}
    />
  </div>

  <div bind:this={keyboard} class="simple-keyboard"></div>
</main>
