import { Channel, invoke } from "@tauri-apps/api/core";

const RADIUS = 150;
const container: HTMLElement | null = document.querySelector('main');
const toast: HTMLDivElement | null = document.querySelector('#toast');
const keyboardLeft: HTMLDivElement | null =
  document.querySelector("#keyboard-left");
const keyboardRight: HTMLDivElement | null =
  document.querySelector("#keyboard-right");

type KeyFocus = [number, number];

const lastFocus: KeyFocus = [-1, -1];
let keyboards: KeyboardType[] = [];
type KeyboardType = {
  left: string[];
  right: string[];
};

type UiEvent =
  | {
      event: "init";
      data: KeyboardType[];
    }
  | {
      event: "showToast";
      data: string;
    }
  | {
    event: "keyboardMode";
    data: boolean;
  }
  | {
      event: "switchKeyboard";
      data: number;
    }
  | {
      event: "keyFocus";
      data: KeyFocus;
    }
  | {
      event: "keyPress";
    };

const eventHandler = (event: UiEvent) => {
  console.log(event);
  switch (event.event) {
    case "init":
      keyboards = event.data;
      setKeyboard(keyboards[0]);
      break;
    case "keyFocus":
      focus(event.data);
      break;
    case "showToast":
      showToast(event.data);
      break;
    case "switchKeyboard":
      setKeyboard(keyboards[event.data]);
      break;
    case "keyPress":
      handleKeyPress();
      break;
    case "keyboardMode":
      handleKeyboardMode(event.data);
  }
};

const onEvent = new Channel<UiEvent>();
onEvent.onmessage = eventHandler;

invoke("initialize", {
  onEvent,
}).then(() => console.log("ready"));

const handleKeyboardMode = (active: boolean) => {
  if (!container) 
    return
  container.style.display = active ? 'block' : 'none';
}

const setKeyboard = (keyboard: KeyboardType) => {
  if (!keyboardLeft || !keyboardRight) return;
  keyboardLeft.innerHTML = "";
  keyboardRight.innerHTML = "";

  const keyboards = [
    [keyboard.left, keyboardLeft],
    [keyboard.right, keyboardRight],
  ] as const;
  keyboards.forEach(([keyboard, parent]) =>
    keyboard.forEach((data, index) => {
      const angle = (index * 360) / keyboard.length;
      const button = document.createElement("button");
      button.innerText = data;
      button.style.transform = `rotate(-${angle}deg) translateX(${RADIUS}px) rotate(${angle}deg)`;
      parent.appendChild(button);
    })
  );
};

const getLastFocusKey = () => {
  if (lastFocus[0] !== -1) return keyboardLeft?.children[lastFocus[0]];
  else if (lastFocus[1] !== -1) return keyboardRight?.children[lastFocus[1]];
  return undefined;
};

const focus = ([left, right]: KeyFocus) => {
  const lastFocusKey = getLastFocusKey();
  lastFocusKey?.classList.remove("focus");

  if (left !== -1) {
    keyboardLeft?.children[left]?.classList.add("focus");
  }
  if (right !== -1) {
    keyboardRight?.children[right]?.classList.add("focus");
  }

  lastFocus[0] = left;
  lastFocus[1] = right;
};

const handleKeyPress = () => {
  const lastFocusKey = getLastFocusKey();
  console.log(lastFocusKey);
  lastFocusKey?.classList.add("pressed");
  setTimeout(() => {
    lastFocusKey?.classList.remove("pressed");
  }, 100);
};

const showToast = (message: string) => {
  if (!toast)
      return;
  toast.innerText = message;
  toast.classList.add('show');
  setTimeout(() => toast.classList.remove('show'), 3000);
}
