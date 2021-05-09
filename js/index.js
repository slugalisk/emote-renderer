// const utilsMod = import("../pkg/index.js");
import * as utilsMod from "../pkg";
import src from "../assets/frames/catJAM4x.png";

(async () => {
  const utils = await utilsMod;

  const data = await (await fetch(src)).arrayBuffer()

  const button = document.createElement("button")
  button.innerText = "run";
  document.body.appendChild(button);

  const frame_width = 112;
  const frame_height = 112;
  const frame_count = 158;

  const canvas = document.createElement("canvas");
  canvas.width = frame_width;
  canvas.height = frame_height;
  document.body.appendChild(canvas);
  const ctx = canvas.getContext("2d");

  const canvas2 = document.createElement("canvas");
  canvas2.width = frame_width;
  canvas2.height = frame_height;
  document.body.appendChild(canvas2);
  const ctx2 = canvas.getContext("2d");

  const img = document.createElement("img");
  document.body.appendChild(img);
  const span = document.createElement("span");
  document.body.appendChild(span);

  const draw = () => {
    let start = Date.now();
    let dec_start = Date.now();
    const rgba = utils.pngDecode(new Uint8Array(data), frame_width, frame_height);
    const dec_time = Date.now() - dec_start

    const frames = [];
    for (let i = 0; i < frame_count; i ++) {
      const start = i * frame_width * frame_height * 4;
      const end = start + frame_width * frame_height * 4;
      frames.push(rgba.subarray(start, end));
    }

    ctx.putImageData(new ImageData(new Uint8ClampedArray(frames[0]), frame_width), 0, 0);

    const enc_start = Date.now();
    // const apng = utils.apngEncodeAll(frames, 0.034, frame_width, frame_height);
    const enc_time = Date.now() - enc_start;
    const total_time = Date.now() - start;

    // img.src = URL.createObjectURL(new Blob([apng], {type: "image/png"}));
    span.innerText = `decode: ${dec_time}ms encode: ${enc_time}ms total: ${total_time}ms`
  };

  button.onclick = draw;
  draw();
})();
