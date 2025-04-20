<!-- YesNoCriterion.vue -->
<template>
  <select v-model="selectedValue" @change="updateValue">
    <option value="">-</option>
    <option value="Yes">Yes</option>
    <option value="No">No</option>
  </select>
</template>

<script setup lang="ts">
import { HardCriterion, YesNo } from '@/types/evaluationCriteria'
import { ref, watch } from 'vue'

const criterion = defineModel<HardCriterion>('criteria')

const selectedValue = ref('')

// Update the parent component when value changes
const updateValue = () => {
  if (selectedValue.value == 'Yes') {
    criterion.value = { YesNo: YesNo.Yes }
  } else if (selectedValue.value == 'No') {
    criterion.value = { YesNo: YesNo.No }
  } else {
    criterion.value = null
  }
}
</script>
