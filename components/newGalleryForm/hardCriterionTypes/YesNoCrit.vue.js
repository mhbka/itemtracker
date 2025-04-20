var __assign = (this && this.__assign) || function () {
    __assign = Object.assign || function(t) {
        for (var s, i = 1, n = arguments.length; i < n; i++) {
            s = arguments[i];
            for (var p in s) if (Object.prototype.hasOwnProperty.call(s, p))
                t[p] = s[p];
        }
        return t;
    };
    return __assign.apply(this, arguments);
};
import { YesNo } from '@/types/evaluationCriteria';
import { ref } from 'vue';
var criterion = defineModel('criteria');
var selectedValue = ref('');
// Update the parent component when value changes
var updateValue = function () {
    if (selectedValue.value == 'Yes') {
        criterion.value = { YesNo: YesNo.Yes };
    }
    else if (selectedValue.value == 'No') {
        criterion.value = { YesNo: YesNo.No };
    }
    else {
        criterion.value = null;
    }
};
debugger; /* PartiallyEnd: #3632/scriptSetup.vue */
var __VLS_defaults = {};
var __VLS_modelEmit = defineEmits();
var __VLS_ctx = {};
var __VLS_components;
var __VLS_directives;
__VLS_asFunctionalElement(__VLS_intrinsicElements.select, __VLS_intrinsicElements.select)(__assign({ onChange: (__VLS_ctx.updateValue) }, { value: (__VLS_ctx.selectedValue) }));
__VLS_asFunctionalElement(__VLS_intrinsicElements.option, __VLS_intrinsicElements.option)({
    value: "",
});
__VLS_asFunctionalElement(__VLS_intrinsicElements.option, __VLS_intrinsicElements.option)({
    value: "Yes",
});
__VLS_asFunctionalElement(__VLS_intrinsicElements.option, __VLS_intrinsicElements.option)({
    value: "No",
});
var __VLS_dollars;
var __VLS_self = (await import('vue')).defineComponent({
    setup: function () {
        return {
            selectedValue: selectedValue,
            updateValue: updateValue,
        };
    },
    __typeEmits: {},
    __typeProps: {},
});
export default (await import('vue')).defineComponent({
    setup: function () {
        return {};
    },
    __typeEmits: {},
    __typeProps: {},
});
; /* PartiallyEnd: #4569/main.vue */
