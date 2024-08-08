import init, { init as bruh, preeti_to_unicode } from "../pkg/preeti_client.js";

init().then(async () => {
  bruh();

  let preeti = document.getElementById("preeti");
  let unicode = document.getElementById("unicode");

  let worker = new Worker("./worker.js", { type: "module" });

  preeti.addEventListener("keyup", (e) => {
    let val = e.target.value;
    let res = val
      .split("\n")
      .map((i) => {
        return preeti_to_unicode(i);
      })
      .join("\n");

    unicode.value = res;
  });

  preeti.addEventListener("keydown", (e) => {
    let val = e.target.value;
    let res = val
      .split("\n")
      .map((i) => {
        return preeti_to_unicode(i);
      })
      .join("\n");

    unicode.value = res;
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
        ..."p-2 flex justify-center items-center text-center w-[20vw] h-20 bg-blue-200 border-2 rounded-xl border-blue-400 hover:bg-blue-300".split(
          " ",
        ),
      );
      loading.remove();
      document.getElementById("container").append(download_el);
    });
  };

  document.getElementById("file_select").addEventListener("change", (e) => {
    const file = e.target.files[0];
    if (file.name.substr(-5) != ".docx") {
      alert("only .docx files are supported");
    } else {
      handle_conversion(file);
    }
  });

  document.getElementById("input-label").addEventListener("dragover", (e) => {
    e.preventDefault();
  });

  document.getElementById("input-label").addEventListener("drop", (e) => {
    e.preventDefault();

    const file = e.dataTransfer.files[0];
    if (file.name.substr(-5) != ".docx") {
      alert("only .docx files are supported");
    } else {
      handle_conversion(file);
    }
  });
});
