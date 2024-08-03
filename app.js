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

  let handle_conversion = async (file) => {
    let buf = preeti_to_unicode_docx(new Uint8Array(file));

    let res = new File([new Uint8Array(buf)], file.name, {
      type: "application/vnd.openxmlformats-officedocument.wordprocessingml.document",
    });
    let url = URL.createObjectURL(res);

    let download_el = document.getElementById("download");
    download_el.href = url;
    download_el.download = res.name;
    download_el.textContent = "Download converted file";

    document.getElementById("container").style.setProperty("display", "none");
    download_el.hidden = false;
  };

  document
    .getElementById("file_select")
    .addEventListener("input", async (e) => {
      let file = await e.target.files[0].arrayBuffer();
      await handle_conversion(file);
    });

  document.getElementById("container").addEventListener("dragover", (e) => {
    e.preventDefault();
  });

  document.getElementById("container").addEventListener("drop", async (e) => {
    e.preventDefault();
    await handle_conversion(await e.dataTransfer.files[0].arrayBuffer());
  });
});
