import init, { preeti_to_unicode_docx } from "../pkg/preeti_client.js";

init();

addEventListener("message", async (e) => {
  try {
    let arr_buf = new Uint8Array(await e.data.arrayBuffer());
    let res_buf = new Uint8Array(preeti_to_unicode_docx(arr_buf));
    postMessage(res_buf);
  } catch (e) {
    //TODO
    console.log(e);
  }
});
