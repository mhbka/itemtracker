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
var __spreadArray = (this && this.__spreadArray) || function (to, from, pack) {
    if (pack || arguments.length === 2) for (var i = 0, l = from.length, ar; i < l; i++) {
        if (ar || !(i in from)) {
            if (!ar) ar = Array.prototype.slice.call(from, 0, i);
            ar[i] = from[i];
        }
    }
    return to.concat(ar || Array.prototype.slice.call(from));
};
import { ref, watch } from 'vue';
import YesNoCriterion from './hardCriterionTypes/YesNoCrit.vue';
import IntCriterion from './hardCriterionTypes/IntCrit.vue';
import FloatCriterion from './hardCriterionTypes/FloatCrit.vue';
import { CriterionType } from '@/types/evaluationCriteria';
var criteria = defineModel('criteria');
var error = defineModel('error');
var errors = ref([null]);
// Returns a default criterion.
function getDefaultCriterion() {
    return {
        question: '',
        criterion_type: CriterionType.YesNo,
    };
}
// Add a new criterion + a placeholder error for it
var addCriterion = function () {
    criteria.value.push(getDefaultCriterion());
    errors.value.push(null);
};
// Remove a specific criterion.
var deleteCriterion = function (index) {
    if (criteria.value.length > 1) {
        criteria.value.splice(index, 1);
        errors.value.splice(index, 1);
    }
};
// emit a single error consisting of all errors, or nothing if there's none
watch(errors, function () {
    var singularError = errors.value
        .map(function (err, index) {
        if (err != null)
            return "".concat(index, ". ").concat(err);
        else
            return err;
    })
        .filter(function (err) { return err != null; })
        .join();
    if (singularError != '')
        error.value = singularError;
    else
        error.value = null;
});
debugger; /* PartiallyEnd: #3632/scriptSetup.vue */
var __VLS_defaults = {};
var __VLS_modelEmit = defineEmits();
var __VLS_ctx = {};
var __VLS_components;
var __VLS_directives;
/** @type {__VLS_StyleScopedClasses['form-group']} */ ;
// CSS variable injection 
// CSS variable injection end 
__VLS_asFunctionalElement(__VLS_intrinsicElements.div, __VLS_intrinsicElements.div)({});
var _loop_1 = function (criterion, index) {
    __VLS_asFunctionalElement(__VLS_intrinsicElements.div, __VLS_intrinsicElements.div)(__assign({ class: "eval-input" }, { key: (index) }));
    __VLS_asFunctionalElement(__VLS_intrinsicElements.div, __VLS_intrinsicElements.div)(__assign({ class: "form-group" }));
    __VLS_asFunctionalElement(__VLS_intrinsicElements.label, __VLS_intrinsicElements.label)({});
    __VLS_asFunctionalElement(__VLS_intrinsicElements.input)({
        value: (criterion.question),
        type: "text",
        placeholder: "Enter question",
    });
    __VLS_asFunctionalElement(__VLS_intrinsicElements.div, __VLS_intrinsicElements.div)(__assign({ class: "form-group" }));
    __VLS_asFunctionalElement(__VLS_intrinsicElements.label, __VLS_intrinsicElements.label)({});
    __VLS_asFunctionalElement(__VLS_intrinsicElements.select, __VLS_intrinsicElements.select)({
        value: (criterion.criterion_type),
    });
    for (var _c = 0, _d = __VLS_getVForSourceType((__VLS_ctx.CriterionType)); _c < _d.length; _c++) {
        var type = _d[_c][0];
        __VLS_asFunctionalElement(__VLS_intrinsicElements.option, __VLS_intrinsicElements.option)({
            key: (type),
            value: (type),
        });
        (type);
    }
    __VLS_asFunctionalElement(__VLS_intrinsicElements.div, __VLS_intrinsicElements.div)(__assign({ class: "form-group" }));
    __VLS_asFunctionalElement(__VLS_intrinsicElements.label, __VLS_intrinsicElements.label)({});
    if (criterion.criterion_type == __VLS_ctx.CriterionType.YesNo) {
        /** @type {[typeof YesNoCriterion, ]} */ ;
        // @ts-ignore
        var __VLS_0 = __VLS_asFunctionalComponent(YesNoCriterion, new YesNoCriterion({
            modelValue: (criterion.hard_criterion),
        }));
        var __VLS_1 = __VLS_0.apply(void 0, __spreadArray([{
                modelValue: (criterion.hard_criterion),
            }], __VLS_functionalComponentArgsRest(__VLS_0), false));
    }
    if (criterion.criterion_type == __VLS_ctx.CriterionType.Float) {
        /** @type {[typeof FloatCriterion, ]} */ ;
        // @ts-ignore
        var __VLS_3 = __VLS_asFunctionalComponent(FloatCriterion, new FloatCriterion({
            criteria: (criterion.hard_criterion),
            error: (__VLS_ctx.errors[index]),
        }));
        var __VLS_4 = __VLS_3.apply(void 0, __spreadArray([{
                criteria: (criterion.hard_criterion),
                error: (__VLS_ctx.errors[index]),
            }], __VLS_functionalComponentArgsRest(__VLS_3), false));
    }
    if (criterion.criterion_type == __VLS_ctx.CriterionType.Int) {
        /** @type {[typeof IntCriterion, ]} */ ;
        // @ts-ignore
        var __VLS_6 = __VLS_asFunctionalComponent(IntCriterion, new IntCriterion({
            criteria: (criterion.hard_criterion),
            error: (__VLS_ctx.errors[index]),
        }));
        var __VLS_7 = __VLS_6.apply(void 0, __spreadArray([{
                criteria: (criterion.hard_criterion),
                error: (__VLS_ctx.errors[index]),
            }], __VLS_functionalComponentArgsRest(__VLS_6), false));
    }
    if (__VLS_ctx.criteria.length > 1) {
        __VLS_asFunctionalElement(__VLS_intrinsicElements.button, __VLS_intrinsicElements.button)(__assign({ onClick: (function () { return __VLS_ctx.deleteCriterion(index); }) }, { type: "button" }));
    }
};
for (var _i = 0, _a = __VLS_getVForSourceType((__VLS_ctx.criteria)); _i < _a.length; _i++) {
    var _b = _a[_i], criterion = _b[0], index = _b[1];
    _loop_1(criterion, index);
}
__VLS_asFunctionalElement(__VLS_intrinsicElements.button, __VLS_intrinsicElements.button)(__assign({ onClick: (__VLS_ctx.addCriterion) }, { type: "button" }));
/** @type {__VLS_StyleScopedClasses['eval-input']} */ ;
/** @type {__VLS_StyleScopedClasses['form-group']} */ ;
/** @type {__VLS_StyleScopedClasses['form-group']} */ ;
/** @type {__VLS_StyleScopedClasses['form-group']} */ ;
var __VLS_dollars;
var __VLS_self = (await import('vue')).defineComponent({
    setup: function () {
        return {
            YesNoCriterion: YesNoCriterion,
            IntCriterion: IntCriterion,
            FloatCriterion: FloatCriterion,
            CriterionType: CriterionType,
            criteria: criteria,
            errors: errors,
            addCriterion: addCriterion,
            deleteCriterion: deleteCriterion,
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
