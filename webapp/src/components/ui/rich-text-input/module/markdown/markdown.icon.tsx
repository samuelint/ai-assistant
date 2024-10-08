import { twMerge } from 'tailwind-merge';


interface Props {
  className?: string
}

export default function MarkdownIcon({ className }: Props) {
  return (
    <svg xmlns="http://www.w3.org/2000/svg" width="208" height="128" viewBox="0 0 208 128" className={twMerge('w-6 h-6', className)}>
      <rect width="198" height="118" x="5" y="5" ry="10" stroke="currentColor" strokeWidth="10" fill="none"/>
      <path fill="currentColor" d="M30 98V30h20l20 25 20-25h20v68H90V59L70 84 50 59v39zm125 0l-30-33h20V30h20v35h20z"/>
    </svg>
  );
}