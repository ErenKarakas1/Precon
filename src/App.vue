<script setup>
  import { ref } from "vue";
  import { invoke } from "@tauri-apps/api/tauri";
  import { dialog } from "@tauri-apps/api";
  import { pictureDir } from "@tauri-apps/api/path";

  const image = ref(null);
  const initialWidth = ref(0);
  const initialHeight = ref(0);
  const width = ref(0);
  const height = ref(0);
  const preserveAspectRatio = ref(true);

  const imageUrl = ref("");
  const convertedImageUrl = ref("");

  const size = ref("");
  const quality = ref(90);

  const onFileChange = (event) => {
    const file = event.target.files[0];

    if (!file) return;

    image.value = file;
    const reader = new FileReader();
    reader.onload = (e) => {
      imageUrl.value = e.target.result;
      const img = new Image();
      img.onload = () => {
        initialWidth.value = img.width;
        initialHeight.value = img.height;
        width.value = initialWidth.value;
        height.value = initialHeight.value;
      };
      img.src = e.target.result;
    };
    reader.readAsDataURL(file);
  };

  function onSizeChange(newWidth, newHeight) {
    if (preserveAspectRatio.value) {
      const aspectRatio = initialWidth.value / initialHeight.value;
      if (width.value !== newWidth && aspectRatio) {          // width changed
        height.value = Math.round(newWidth / aspectRatio);
      } else if (height.value !== newHeight && aspectRatio) { // height changed
        width.value = Math.round(newHeight * aspectRatio);
      }
    } else {
      width.value = newWidth;
      height.value = newHeight;
    }
  };

  async function processImage(processFunction) {
    return new Promise((resolve, reject) => {
      const reader = new FileReader();
      reader.onload = async (e) => {
        const arrayBuffer = e.target.result;
        const image_blob = Array.from(new Uint8Array(arrayBuffer));
        const params = { 
          imageBlob: image_blob, 
          width: Number(width.value), 
          height: Number(height.value), 
          quality: Number(quality.value) 
        };
        try {
          const res = await processFunction(params);
          resolve(res);
        } catch (err) {
          reject(err);
        }
      };
      reader.onerror = reject;
      reader.readAsArrayBuffer(image.value);
    });
  }

  async function preview() {
    console.log("Previewing image with size: " + width.value + "x" + height.value + " and quality: " + quality.value);
    const res = await processImage((params) => invoke("preview", params));
    convertedImageUrl.value = res[0];
    size.value = res[1];
    console.log("Previewed image size: " + size.value);
  }

  async function convert() {
    const pictureDirPath = await pictureDir();
    const savePath = await dialog.save({
      defaultPath: pictureDirPath + "/converted.jpg",
      filters: [{ name: 'Images', extensions: ["jpg"]}]
    });

    if (!savePath) return;

    const res = await processImage((params) => invoke("convert", { ...params, savePath: savePath }));
    console.log("Saved converted image to: " + savePath + " with size: " + res);
  }
</script>

<template>
  <v-container>
    <v-row>
      <v-col class="side-column">
        <div style="max-width: 100vw; max-height: 100vh; overflow: hidden;">
          <v-img
            min-height="200"
            :src="imageUrl"
            aspect-ratio="1"
            contain
          >
          </v-img>
        </div>
        <v-file-input
          accept="image/*"
          label="Select an image"
          variant="outlined"
          prepend-icon="mdi-image"
          show-size
          @change="onFileChange"
        ></v-file-input>
      </v-col>
      <v-col class="buttons-column" cols="3">
        <v-row class="d-flex flex-column">
          <v-row class="resize-btns">
            <v-text-field
              class="resize-btn"
              v-model="width"
              label="Width"
              :value="width"
              :min=1
              type="number"
              variant="outlined"
              suffix="px"
              @input="e => onSizeChange(Number(e.target.value), height)"
            ></v-text-field>
            <v-text-field
              class="resize-btn"
              v-model="height"
              label="Height"
              :value="height"
              :min=1
              type="number"
              variant="outlined"
              suffix="px"
              @input="e => onSizeChange(width, Number(e.target.value))"
            ></v-text-field>
          </v-row>
          <v-checkbox
            v-model="preserveAspectRatio"
            :label="`Preserve aspect ratio (${initialWidth}:${initialHeight})`"
          ></v-checkbox>
        </v-row>
        <v-row>
          <v-text-field
            v-model="quality"
            label="Quality"
            :value="quality"
            type="number"
            hide-details
            variant="outlined"
            suffix="%"
          ></v-text-field>
        </v-row>
        <v-row class="d-flex justify-space-around">
          <v-btn class="preview-btn" @click="preview"> Preview </v-btn>
          <v-btn class="convert-btn" @click="convert"> Convert </v-btn>
        </v-row>
      </v-col>
      <v-col class="side-column">
        <div style="max-width: 100vw; max-height: 100vh; overflow: hidden">
          <v-img
            min-height="200"
            lazy-src=""
            :src="convertedImageUrl"
            aspect-ratio="1"
            contain
          >
            <template v-slot:placeholder>
              <div class="d-flex align-center justify-center fill-height">
                <v-progress-circular
                  color="grey-lighten-4"
                />
              </div>
            </template>
          </v-img>
        </div>
        <v-text-field
          v-model="size"
          label="Size"
          :value="size"
          type="text"
          readonly
          variant="outlined"
        ></v-text-field>
      </v-col>
    </v-row>
  </v-container>
</template>

<style scoped>
  .v-container {
    padding-top: 15vh;
    display: flex;
    flex-direction: column;
    justify-content: center;
    text-align: center;
  }
  .v-col, .v-row {
    align-content: center;
    justify-content: center;
  }
  .side-column {
    display: flex;
    flex-direction: column;
    row-gap: 2rem;
  }
  .buttons-column {
    display: flex;
    flex-direction: column;
    row-gap: 1rem;
    margin: 0 5rem;
  }
  .resize-btns {
    column-gap: 1rem;
  }
  .resize-btn {
    max-width: 45%;
  }
  .preview-btn, .convert-btn {
    background-color: #0582CA;
    border: 1px solid #FFFFFF;
    border-radius: 10px;
    color: #FFFFFF;
    padding: 1rem 2rem 2rem 2rem;
    transition-duration: 0.1s;
  }
  .preview-btn:hover, .convert-btn:hover {
    background-color: #00CAFE;
  }
</style>
