console.log("Hello from wry");

const colorIt = (color) => {
  // messages background
  const path = '//*[@id="__next"]/div/div[1]/main/div[1]/div/div/div';
  xpathRecolor(color, path);

  // messages
  Array.from(document.querySelectorAll(".group")).map(
    (x) => (x.style.backgroundColor = color)
  );

  // header
  document.querySelector(".sticky").style.backgroundColor = color;

  // input box
  const inputBoxPath =
    '//*[@id="__next"]/div/div[1]/main/div[2]/form/div/div[last()]';
  xpathRecolor(color, inputBoxPath);

  const lastItemPath =
    '//*[@id="__next"]/div/div[1]/main/div[1]/div/div/div/div[last()]';
  xpathRecolor(color, lastItemPath);

  // footer overlay
  document.querySelector(".bottom-0").style.backgroundImage = "none";

  // round icons
  Array.from(document.querySelectorAll(".rounded-sm")).map((x) => {
    x.style.borderRadius = "50%";
  });
};

const processMessages = () => {
  const messages = Array.from(
    document.querySelector("main").querySelectorAll(".text-base")
  ).map((data) => ({
    color:
      data.querySelector("path").getAttribute("fill") == null
        ? "human"
        : "openai",
    text: data.querySelector(".w-full").innerText,
  }));

  // window.ipc.postMessage('new-window')
  window.ipc.postMessage("change-title " + JSON.stringify(messages));
};

const xpathRecolor = (color, path2) => {
  try {
    const paragraphCount2 = document.evaluate(
      path2,
      document,
      null,
      XPathResult.ANY_TYPE,
      null
    );
    const item2 = paragraphCount2.iterateNext();
    item2.style.backgroundColor = color;
  } catch (error) {
    console.log("failed to recolor specific element", path2);
  }
};

window.addEventListener("load", (event) => {
  const tryColor = () => {
    try {
      colorIt("#111");
    } catch (error) {
      console.log("failed to color it");
    }
  };

  // try initial color real quick after load
  setTimeout(tryColor, 100);

  // then try to color it every 250ms
  setInterval(() => {
    tryColor();
    processMessages();
  }, 250);
});

// TODO: figure out how to listen to the correct events to know when to process messages
// 
// window.addEventListener("animationcancel", (event) => {
//   processMessages();
// });

// Object.keys(window).forEach((key) => {
//   if (/^on/.test(key)) {
//     window.addEventListener(key.slice(2), (event) => {
//       // if (event.type == "animationend") {
//       //   console.log("just finished responding")
//       //   // colorIt("#111");
//       // }
//       console.log(event.type);
//     });
//   }
// });
