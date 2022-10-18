<script setup lang="ts">
import { ref } from 'vue'
import nthPrime from 'nth-prime'
import { greet, nth_prime, nth_fibo } from 'rust-num_ext'

// import { parse } from 'markdown-wasm'
// console.log(parse("# hello\n*world*"))

// greet('wasm and vue')

const idx = ref(0)
const cal = () => {
  calJS()
  calRS()
}

const jsPrime = ref(0)
const jsTimeRange = ref(0)
const calJS = () => {
  let startTime = performance.now()
  jsPrime.value = nthPrime(idx.value)
  jsTimeRange.value = performance.now() - startTime
}

const rsPrime = ref(0)
const rsTimeRange = ref(0)
const calRS = () => {
  let startTime = performance.now()
  rsPrime.value = nth_prime(idx.value)
  rsTimeRange.value = performance.now() - startTime
}

const idxFibo = ref(0)
const calFibo = () => {
  calFiboJS()
  calFiboRs()
}

const jsFibo = ref(0)
const jsFiboTimeRange = ref(0)
const calFiboJS = () => {
  let startTime = performance.now()

  let n1 = 0, n2 = 1, nextTerm;
  for (let i = 1; i < idxFibo.value; i++) {
    nextTerm = n1 + n2;
    n1 = n2;
    n2 = nextTerm;
  }
  jsFibo.value = n2

  jsFiboTimeRange.value = performance.now() - startTime
}

const rsFibo = ref(0)
const rsFiboTimeRange = ref(0)
const calFiboRs = () => {
  let startTime = performance.now()

  rsFibo.value = nth_fibo(idxFibo.value)

  rsFiboTimeRange.value = performance.now() - startTime
}

</script>

<template>
  <div class="num-ext">
    <div>
      <h1>Prime Number</h1>
      <div>
        Index: <input v-model="idx" />
        <button @click="cal">Ger Prime</button>
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

    <div style="margin-top: 20px">
      <h1>Fibonacci</h1>
      <div>
        Index: <input v-model="idxFibo" />
        <button @click="calFibo">Ger Fibonacci</button>
      </div>
      <div style="display: flex; justify-content: space-around">
        <div>
          <h2><button @click="calFiboJS">Just Cal</button> By JS</h2>
          <div>
            Result: {{ jsFibo }}
          </div>
          <div>
            Time Range: {{ jsFiboTimeRange }} ms
          </div>
        </div>
        <div>
          <h2><button @click="calFiboRs">Just Cal</button> By Rust</h2>
          <div>
            Result: {{ rsFibo }}
          </div>
          <div>
            Time Range: {{ rsFiboTimeRange }} ms
          </div>
        </div>
        <div>
          <h2>By Ruby</h2>
        </div>
      </div>
    </div>
  </div>
</template>
