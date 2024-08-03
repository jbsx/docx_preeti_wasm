import init, {
  init as bruh,
  preeti_to_unicode,
  preeti_to_unicode_docx,
} from "./pkg/preeti_client.js";

init().then(async () => {
  bruh();

  let unicode = document.getElementById("unicode");
  let preeti = document.getElementById("preeti");

  unicode.addEventListener("keyup", (e) => {
    let val = e.target.value;
    let res = val
      .split("\n")
      .map((i) => {
        return preeti_to_unicode(i);
      })
      .join("\n");

    preeti.value = res;
  });

  unicode.addEventListener("keydown", (e) => {
    let val = e.target.value;
    let res = val
      .split("\n")
      .map((i) => {
        return preeti_to_unicode(i);
      })
      .join("\n");

    preeti.value = res;
  });

  document.getElementById("bruh").addEventListener("change", async (e) => {
    let file = await e.target.files[0].arrayBuffer();
    let buf = preeti_to_unicode_docx(new Uint8Array(file));

    let res = new File([new Uint8Array(buf)], e.target.files[0].name, {
      type: "application/vnd.openxmlformats-officedocument.wordprocessingml.document",
    });
    let url = URL.createObjectURL(res);

    let download_el = document.createElement("a");
    download_el.href = url;
    download_el.download = res.name;
    download_el.textContent = "Download converted file";
    document.querySelector("body").append(download_el);
  });
});
