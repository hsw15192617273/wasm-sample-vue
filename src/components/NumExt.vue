<script setup lang="ts">
import { ref } from 'vue'
import nthPrime from 'nth-prime'
import { greet, nth_prime } from 'rust-num_ext'


// greet('wasm and vue')

const i = ref(0)
const jsPrime = ref(0)
const jsTimeRange = ref(0)
const rsPrime = ref(0)
const rsTimeRange = ref(0)
const rbPrime = ref(0)

const onClick = () => {
  calJS()
  calRS()
}

const calJS = () => {
  let startTime = performance.now()
  jsPrime.value = nthPrime(i.value)
  jsTimeRange.value = performance.now() - startTime
}

const calRS = () => {
  let startTime = performance.now()
  rsPrime.value = nth_prime(i.value)
  rsTimeRange.value = performance.now() - startTime
}
</script>

<template>
  <div class="num-ext">
    <div>
      Index: <input v-model="i" />
      <button @click="onClick">Ger Prime</button>
    </div>
    <div style="display: flex; justify-content: space-around;">
      <div>
        <h2><button @click="calJS">Just Cal</button> By JS</h2>
        <div>
          Result: {{ jsPrime }}
        </div>
        <div>
          Time Range: {{ jsTimeRange }} ms
        </div>
      </div>
      <div>
        <h2><button @click="calRS">Just Cal</button> By Rust</h2>
        <div>
          Result: {{ rsPrime }}
        </div>
        <div>
          Time Range: {{ rsTimeRange }} ms
        </div>
      </div>
      <div>
        <h2>By Ruby</h2>
      </div>
    </div>
  </div>
</template>
