<template>
  <div>
    <div class="eval-input" v-for="(criterion, index) in criteria" :key="index">
      <div class="form-group">
        <label>Question</label>
        <input v-model="criterion.question" type="text" placeholder="Enter question" />
      </div>

      <div class="form-group">
        <label>Criterion Type</label>
        <select v-model="criterion.criterion_type">
          <option v-for="type in CriterionType" :key="type" :value="type">{{ type }}</option>
        </select>
      </div>

      <div class="form-group">
        <label>Hard Criterion (optional)</label>
        <YesNoCriterion 
          v-if="criterion.criterion_type == CriterionType.YesNo" 
          v-model="criterion.hard_criterion"
        />
        <FloatCriterion 
          v-if="criterion.criterion_type == CriterionType.Float" 
          v-model:criteria="criterion.hard_criterion"
          v-model:error="errors[index]"
        />
        <IntCriterion   
          v-if="criterion.criterion_type == CriterionType.Int" 
          v-model:criteria="criterion.hard_criterion"
          v-model:error="errors[index]"
        />
      </div>

      <button v-if="criteria.length > 1" type="button" @click="() => deleteCriterion(index)">Delete Criterion</button>
    </div>

    <button type="button" @click="addCriterion">Add Criterion</button>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, computed, watch } from 'vue';
import YesNoCriterion from './hardCriterionTypes/YesNoCrit.vue';;
import IntCriterion from './hardCriterionTypes/IntCrit.vue';
import FloatCriterion from './hardCriterionTypes/FloatCrit.vue';
import { Criterion, CriterionType, HardCriterion } from '@/types/evaluationCriteria';

let criteria = defineModel<Criterion[]>('criteria');
let error = defineModel<string>('error');

const errors = ref<string[]>([null]);

// Returns a default criterion.
function getDefaultCriterion(): Criterion {
  return {
    question: '',
    criterion_type: CriterionType.YesNo,
  };
}

// Add a new criterion + a placeholder error for it
const addCriterion = () => {
  criteria.value.push(getDefaultCriterion());
  errors.value.push(null);
}

// Remove a specific criterion.
const deleteCriterion = (index) => {
  if (criteria.value.length > 1) {  
    criteria.value.splice(index, 1);
    errors.value.splice(index, 1);
  }
}

// emit a single error consisting of all errors, or nothing if there's none
watch(errors, () => {
  const singularError = errors.value
    .map((err, index) => {
      if (err != null)
        return `${index}. ${err}`;
      else 
        return err;
    })
    .filter((err) => err != null)
    .join();
  if (singularError != '') 
    error.value = singularError;
  else
    error.value = null;
})
</script>

<style lang="css" scoped>
.eval-input {
  margin-bottom: 1rem;
  padding-bottom: 1rem;
  flex-direction: column;
  border-bottom: 1px solid;
}

.form-group {
  margin-bottom: 1rem;
  display: flex;
  flex-direction: column;
}

.form-group label {
font-weight: 500;
margin-bottom: 0.25rem;
}

</style>