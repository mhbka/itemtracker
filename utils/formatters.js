import { format } from 'date-fns/format';
export function formatUnixTimestamp(timestamp) {
    if (timestamp) {
        return new Date(timestamp * 1000).toLocaleString();
    }
    return '--';
}
export function getZeroedNaiveDatetime() {
    return format(new Date(0), "yyyy-M-d'T'HH:mm:ss");
}
export function formatPrice(price) {
    return new Intl.NumberFormat('en-US', {
        style: 'currency',
        currency: 'JPY',
    }).format(price);
}
export function formatCriterionAnswer(answer) {
    if ('YesNo' in answer)
        return answer.YesNo;
    else if ('YesNoUncertain' in answer)
        return answer.YesNoUncertain;
    else if ('Int' in answer)
        return answer.Int.toString();
    else if ('Float' in answer)
        return answer.Float.toString();
    else if ('OpenEnded' in answer)
        return answer.OpenEnded;
}
export function formatHardCriterion(crit) {
    if ('YesNo' in crit)
        return crit.YesNo;
    else if ('Int' in crit) {
        if ('LessThan' in crit.Int)
            return "less than ".concat(crit.Int.LessThan);
        else if ('MoreThan' in crit.Int)
            return "more than ".concat(crit.Int.MoreThan);
        else if ('Equal' in crit.Int)
            return "equal ".concat(crit.Int.Equal);
        else if ('Between' in crit.Int)
            return "between ".concat(crit.Int.Between[0], " and ").concat(crit.Int.Between[1]);
    }
    else if ('Float' in crit) {
        if ('LessThan' in crit.Float)
            return "less than ".concat(crit.Float.LessThan);
        else if ('MoreThan' in crit.Float)
            return "more than ".concat(crit.Float.MoreThan);
        else if ('Equal' in crit.Float)
            return "equal ".concat(crit.Float.Equal);
        else if ('Between' in crit.Float)
            return "between ".concat(crit.Float.Between[0], " and ").concat(crit.Float.Between[1]);
    }
    else
        return '-';
}
export function truncateText(text, maxLength) {
    if (maxLength === void 0) { maxLength = 50; }
    if (text.length <= maxLength)
        return text;
    return text.substring(0, maxLength) + '...';
}
