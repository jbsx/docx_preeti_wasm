import init, {
  init as bruh,
  unicode_to_preeti,
  unicode_to_preeti_docx,
} from "../pkg/preeti_client.js";

init().then(async () => {
  bruh();

  let unicode = document.getElementById("unicode");
  let preeti = document.getElementById("preeti");

  unicode.addEventListener("keyup", (e) => {
    let val = e.target.value;
    let res = val
      .split("\n")
      .map((i) => {
        return unicode_to_preeti(i);
      })
      .join("\n");

    preeti.value = res;
  });

  unicode.addEventListener("keydown", (e) => {
    let val = e.target.value;
    let res = val
      .split("\n")
      .map((i) => {
        return unicode_to_preeti(i);
      })
      .join("\n");

    preeti.value = res;
  });

  let handle_conversion = async (file) => {
    let buf = unicode_to_preeti_docx(new Uint8Array(await file.arrayBuffer()));

    let res = new File([new Uint8Array(buf)], file.name, {
      type: "application/vnd.openxmlformats-officedocument.wordprocessingml.document",
    });
    let url = URL.createObjectURL(res);
    let download_el = document.createElement("a");
    console.log(file);
    download_el.href = url;
    download_el.download = file.name;
    download_el.textContent = "Download converted file";

    document.getElementById("input-label").classList.add("hidden");
    download_el.classList.add(
      ..."flex justify-center items-center w-[20vw] h-20 bg-blue-200 border-2 rounded-xl border-blue-400 hover:bg-blue-300".split(
        " ",
      ),
    );

    document.getElementById("container").append(download_el);
  };

  document
    .getElementById("file_select")
    .addEventListener("change", async (e) => {
      let file = e.target.files[0];
      await handle_conversion(file);
    });

  document.getElementById("input-label").addEventListener("dragover", (e) => {
    e.preventDefault();
  });

  document.getElementById("input-label").addEventListener("drop", async (e) => {
    e.preventDefault();
    await handle_conversion(e.dataTransfer.files[0]);
  });
});
