import type { UnixUtcDateTime, CriterionAnswer } from '@/types/galleries';

export function formatUnixTimestamp(timestamp: UnixUtcDateTime): string {
  return new Date(timestamp * 1000).toLocaleString();
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