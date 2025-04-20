<template>
  <div>
    <div></div>
    <select v-model="operator" @change="updateValue">
      <option value="">-</option>
      <option value="LessThan">Less than</option>
      <option value="Equal">Equal to</option>
      <option value="MoreThan">More than</option>
      <option value="Between">Between</option>
    </select>

    <div v-if="operator === 'Between'">
      <input type="number" v-model.number="value1" @input="updateValue" placeholder="Min" />
      <span>and</span>
      <input type="number" v-model.number="value2" @input="updateValue" placeholder="Max" />
    </div>

    <div v-else-if="operator">
      <input type="number" v-model.number="value1" @input="updateValue" placeholder="Value" />
    </div>
  </div>
</template>

<script setup lang="ts">
import { HardCriterion, IntHardCriterion } from '@/types/evaluationCriteria'
import { ref } from 'vue'

const error = defineModel<string>('error')
const criterion = defineModel<HardCriterion>('criteria')

const operator = ref('')
const value1 = ref(null)
const value2 = ref(null)

// validates values before emitting. if invalid, an error string is emitted too.
const updateValue = () => {
  let criterionValue: IntHardCriterion

  if (!operator.value) {
    criterionValue = null
  } else if (operator.value === 'Between') {
    if (value1.value !== null && value2.value !== null) {
      if (value1.value < value2.value) {
        if (Number.isInteger(value1.value) && Number.isInteger(value2.value))
          criterionValue = { Between: [value1.value, value2.value] }
        else error.value = 'Ensure both values are integers.'
      } else error.value = 'The first value must be less than the second.'
    } else error.value = 'Please set the 2 values.'
  } else if (value1.value !== null) {
    if (operator.value === 'LessThan') criterionValue = { LessThan: value1.value }
    else if (operator.value === 'Equal') criterionValue = { Equal: value1.value }
    else if (operator.value === 'MoreThan') criterionValue = { MoreThan: value1.value }
    else error.value = 'An invalid type of hard criterion was set.'
  } else error.value = 'Please set a value.'

  criterion.value = { Float: criterionValue }
}
</script>
