import init, { preeti_to_unicode_docx } from "../pkg/preeti_client.js";

init();

addEventListener("message", async (e) => {
  try {
    let { file, loading } = e.data;
    let arr_buf = new Uint8Array(await file.arrayBuffer());
    let res_buf = new Uint8Array(preeti_to_unicode_docx(arr_buf, loading));
    postMessage(res_buf);
  } catch (e) {
    //TODO
    console.log(e);
  }
});
