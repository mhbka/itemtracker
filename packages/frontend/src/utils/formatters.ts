import type { UnixUtcDateTime, CriterionAnswer } from '@/types/galleries';
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
    currency: 'USD'
  }).format(price);
}

export function formatCriterionAnswer(answer: CriterionAnswer): string {
  switch (answer.type) {
    case 'YesNo':
      return answer.value;
    case 'YesNoUncertain':
      return answer.value;
    case 'Int':
      return answer.value.toString();
    case 'Float':
      return answer.value.toFixed(2);
    case 'OpenEnded':
      return answer.value;
    default:
      return 'Unknown answer type';
  }
}

export function truncateText(text: string, maxLength: number = 50): string {
  if (text.length <= maxLength) return text;
  return text.substring(0, maxLength) + '...';
}