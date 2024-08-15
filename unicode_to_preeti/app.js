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
    let loading = document.getElementById("loading");
    loading.classList.remove("hidden");

    worker.postMessage(file);

    worker.addEventListener("message", (res_buf) => {
      let res = new File([res_buf.data], file.name, {
        type: "application/vnd.openxmlformats-officedocument.wordprocessingml.document",
      });

      let url = URL.createObjectURL(res);
      let download_el = document.getElementById("btn");
      download_el.href = url;
      download_el.download = res.name;

      download_el.classList.remove("hidden");
      loading.classList.add("hidden");
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
