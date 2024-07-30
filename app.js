import init, { init as bruh, preeti_to_unicode } from "./pkg/preeti_client.js";

init().then(async () => {
  bruh();

  let fetch_preeti = await fetch("./preeti.json");
  let data = await fetch_preeti.json();
  let unicode = document.getElementById("unicode");
  let preeti = document.getElementById("preeti");

  unicode.addEventListener("keyup", (e) => {
    let val = e.target.value;
    let res = val
      .split("\n")
      .map((i) => {
        return preeti_to_unicode(i, JSON.stringify(data));
      })
      .join("\n");

    preeti.value = res;
  });

  unicode.addEventListener("keydown", (e) => {
    let val = e.target.value;
    let res = val
      .split("\n")
      .map((i) => {
        return preeti_to_unicode(i, JSON.stringify(data));
      })
      .join("\n");

    preeti.value = res;
  });
});
