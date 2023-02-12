import { useState } from "react";
import { createRoot } from "react-dom/client";
import "../global.css";
import { sendMsgToBackground } from "../messenger";

function Expanded() {
  const [block, setBlock] = useState(0);
  function getBlockNumber() {
    sendMsgToBackground("BLOCK", { message: block.toString() });
    setBlock(block + 1);
  }

  return (
    <div>
      Expanded!
      <button
        onClick={() => {
          getBlockNumber();
        }}
      >
        Get
      </button>
      <p>Block Number: {block}</p>
    </div>
  );
}

function init() {
  const appContainer = document.querySelector("#app-container");
  if (!appContainer) {
    throw new Error("Can not find #app-container");
  }
  const root = createRoot(appContainer);
  root.render(<Expanded />);
}

init();
