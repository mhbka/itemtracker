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
import { ref } from 'vue';
var error = defineModel('error');
var criterion = defineModel('criteria');
var operator = ref('');
var value1 = ref(null);
var value2 = ref(null);
// validates values before emitting. if invalid, an error string is emitted too.
var updateValue = function () {
    var criterionValue;
    if (!operator.value) {
        criterionValue = null;
    }
    else if (operator.value === 'Between') {
        if (value1.value !== null && value2.value !== null) {
            if (value1.value < value2.value)
                criterionValue = { Between: [value1.value, value2.value] };
            else
                error.value = 'The first value must be less than the second.';
        }
        else
            error.value = 'Please set the 2 values.';
    }
    else if (value1.value !== null) {
        if (operator.value === 'LessThan')
            criterionValue = { LessThan: value1.value };
        else if (operator.value === 'Equal')
            criterionValue = { Equal: value1.value };
        else if (operator.value === 'MoreThan')
            criterionValue = { MoreThan: value1.value };
        else
            error.value = 'An invalid type of hard criterion was set.';
    }
    else
        error.value = 'Please set a value.';
    criterion.value = { Float: criterionValue };
};
debugger; /* PartiallyEnd: #3632/scriptSetup.vue */
var __VLS_defaults = {};
var __VLS_modelEmit = defineEmits();
var __VLS_ctx = {};
var __VLS_components;
var __VLS_directives;
__VLS_asFunctionalElement(__VLS_intrinsicElements.div, __VLS_intrinsicElements.div)({});
__VLS_asFunctionalElement(__VLS_intrinsicElements.div, __VLS_intrinsicElements.div)({});
__VLS_asFunctionalElement(__VLS_intrinsicElements.select, __VLS_intrinsicElements.select)(__assign({ onChange: (__VLS_ctx.updateValue) }, { value: (__VLS_ctx.operator) }));
__VLS_asFunctionalElement(__VLS_intrinsicElements.option, __VLS_intrinsicElements.option)({
    value: "",
});
__VLS_asFunctionalElement(__VLS_intrinsicElements.option, __VLS_intrinsicElements.option)({
    value: "LessThan",
});
__VLS_asFunctionalElement(__VLS_intrinsicElements.option, __VLS_intrinsicElements.option)({
    value: "Equal",
});
__VLS_asFunctionalElement(__VLS_intrinsicElements.option, __VLS_intrinsicElements.option)({
    value: "MoreThan",
});
__VLS_asFunctionalElement(__VLS_intrinsicElements.option, __VLS_intrinsicElements.option)({
    value: "Between",
});
if (__VLS_ctx.operator === 'Between') {
    __VLS_asFunctionalElement(__VLS_intrinsicElements.div, __VLS_intrinsicElements.div)({});
    __VLS_asFunctionalElement(__VLS_intrinsicElements.input)(__assign({ onInput: (__VLS_ctx.updateValue) }, { type: "number", placeholder: "Min" }));
    (__VLS_ctx.value1);
    __VLS_asFunctionalElement(__VLS_intrinsicElements.span, __VLS_intrinsicElements.span)({});
    __VLS_asFunctionalElement(__VLS_intrinsicElements.input)(__assign({ onInput: (__VLS_ctx.updateValue) }, { type: "number", placeholder: "Max" }));
    (__VLS_ctx.value2);
}
else if (__VLS_ctx.operator) {
    __VLS_asFunctionalElement(__VLS_intrinsicElements.div, __VLS_intrinsicElements.div)({});
    __VLS_asFunctionalElement(__VLS_intrinsicElements.input)(__assign({ onInput: (__VLS_ctx.updateValue) }, { type: "number", placeholder: "Value" }));
    (__VLS_ctx.value1);
}
var __VLS_dollars;
var __VLS_self = (await import('vue')).defineComponent({
    setup: function () {
        return {
            operator: operator,
            value1: value1,
            value2: value2,
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
