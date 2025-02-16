<script setup>
  import { ref } from "vue";
  import { invoke } from "@tauri-apps/api/core";
  import { pictureDir } from "@tauri-apps/api/path";
  import { save } from "@tauri-apps/plugin-dialog"

  const image = ref(null);
  const initialWidth = ref(0);
  const initialHeight = ref(0);
  const width = ref(0);
  const height = ref(0);
  const preserveAspectRatio = ref(true);

  const imageUrl = ref("");
  const convertedImageUrl = ref("");
  const isLoading = ref(false);

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
    if (preserveAspectRatio.value && initialWidth.value && initialHeight.value) {
      const aspectRatio = initialWidth.value / initialHeight.value;
      
      if (width.value !== newWidth) {
        height.value = Math.round(newWidth / aspectRatio);
      } else if (height.value !== newHeight) {
        width.value = Math.round(newHeight * aspectRatio);
      }
    } else {
      width.value = newWidth;
      height.value = newHeight;
    }
  };

  async function processImage(action, additionalParams = {}) {
    if (!image.value) {
      throw new Error("No image selected.");
    }

    const arrayBuffer = await image.value.arrayBuffer();
    const imageBlob = new Uint8Array(arrayBuffer);

    const params = {
      imageBlob: imageBlob,
      width: Number(width.value),
      height: Number(height.value),
      quality: Number(quality.value),
      ...additionalParams,
    };

    return await invoke(action, params);
  }

  async function preview() {
    isLoading.value = true;

    try {
      const res = await processImage('preview');
      convertedImageUrl.value = res[0];
      size.value = res[1];
    } catch (error) {
      console.error("Failed to preview image:", error);
    } finally {
      isLoading.value = false;
    }
  }

  async function convert() {
    isLoading.value = true;
    const pictureDirPath = await pictureDir();
    const savePath = await save({
      defaultPath: `${pictureDirPath}/converted.jpg`,
      filters: [{ name: 'Images', extensions: ["jpg"]}]
    });

    if (!savePath) {
      isLoading.value = false;
      return;
    }

    try {
      const res = await processImage('convert', { savePath: savePath });
    } catch (error) {
      console.error("Failed to convert image:", error);
    } finally {
      isLoading.value = false;
    }
  }
</script>

<template>
  <v-container>
    <v-row>
      <v-col class="side-column">
        <div class="img-container">
          <v-img
            min-height="200"
            :src="imageUrl"
            aspect-ratio="1"
            contain
          >
            <template #placeholder>
              <v-row class="fill-height ma-0" align="center" justify="center">
                <span>Waiting for the user to upload an image...</span>
              </v-row>
            </template>
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
            type="number"
            hide-details
            variant="outlined"
            suffix="%"
          ></v-text-field>
        </v-row>
        <v-row class="d-flex justify-space-around">
          <v-btn class="preview-btn" @click="preview" :disabled="isLoading"> Preview </v-btn>
          <v-btn class="convert-btn" @click="convert" :disabled="isLoading"> Convert </v-btn>
        </v-row>
      </v-col>
      <v-col class="side-column">
        <div class="img-container">
          <v-img
            min-height="200"
            lazy-src=""
            :src="convertedImageUrl"
            aspect-ratio="1"
            contain
          >
            <template v-if="isLoading" #placeholder>
              <v-row class="fill-height ma-0" align="center" justify="center">
                <v-progress-circular indeterminate></v-progress-circular>
              </v-row>
            </template>
            <template v-else #placeholder>
              <v-row class="fill-height ma-0" align="center" justify="center">
                <span>No preview requested yet</span>
              </v-row>
            </template>
          </v-img>
        </div>
        <v-text-field
          v-model="size"
          label="Size"
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
  .img-container {
    max-width: 100vw;
    max-height: 100vh;
    border: 1px solid #000000;
    overflow: hidden;
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
