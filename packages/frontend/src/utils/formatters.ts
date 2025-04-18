import { CriterionAnswer } from '@/types/evaluationCriteria';
import type { UnixUtcDateTime } from '@/types/galleries';
import { format } from 'date-fns/format';

export function formatUnixTimestamp(timestamp?: UnixUtcDateTime): string {
  if (timestamp) {
    return new Date(timestamp * 1000).toLocaleString();
  }
  return '--';
}

export function getZeroedNaiveDatetime(): string {
  return format(new Date(0), 'yyyy-M-d\'T\'HH:mm:ss');
}

export function formatPrice(price: number): string {
  return new Intl.NumberFormat('en-US', {
    style: 'currency',
    currency: 'JPY'
  }).format(price);
}

export function formatCriterionAnswer(answer: CriterionAnswer): string {
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

export function truncateText(text: string, maxLength: number = 50): string {
  if (text.length <= maxLength) return text;
  return text.substring(0, maxLength) + '...';
}