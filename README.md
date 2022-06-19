# raw-rs
raw画像を処理する

## 全体の流れ
raw-file -> decoder(rawloader crate) -> processor(raw, rgb, yuv) -> png(jpeg)-file(image crate)

decoderでrawファイルの解析をする  

