import init, { init as bruh, unicode_to_preeti } from "../pkg/preeti_client.js";

init().then(async () => {
  bruh();

  let unicode = document.getElementById("unicode");
  let preeti = document.getElementById("preeti");

  let worker = new Worker("worker.js", { type: "module" });

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

  let handle_conversion = (file) => {
    document.getElementById("input-label").classList.add("hidden");
    let loading = document.createElement("img");
    loading.src = "../loading.svg";
    document.getElementById("container").append(loading);

    worker.postMessage(file);

    worker.addEventListener("message", (res_buf) => {
      let res = new File([res_buf.data], file.name, {
        type: "application/vnd.openxmlformats-officedocument.wordprocessingml.document",
      });
      let url = URL.createObjectURL(res);
      let download_el = document.createElement("a");
      download_el.href = url;
      download_el.download = res.name;
      download_el.textContent = "Download converted file";

      download_el.classList.add(
        ..."flex justify-center items-center text-center w-[20vw] h-20 bg-blue-200 border-2 rounded-xl border-blue-400 hover:bg-blue-300".split(
          " ",
        ),
      );
      loading.remove();
      document.getElementById("container").append(download_el);
    });
  };

  document.getElementById("file_select").addEventListener("change", (e) => {
    handle_conversion(e.target.files[0]);
  });

  document.getElementById("input-label").addEventListener("dragover", (e) => {
    e.preventDefault();
  });

  document.getElementById("input-label").addEventListener("drop", (e) => {
    e.preventDefault();

    handle_conversion(e.dataTransfer.files[0]);
  });
});
