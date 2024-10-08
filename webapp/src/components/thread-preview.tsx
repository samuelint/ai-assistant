import { Link } from 'wouter';
import { type ThreadPreviewDto } from '@/lib/assistant/thread.type';
import { cn } from '@/lib/utils';
import { buttonVariants } from './ui/button';
import { ThreadPreviewContextMenu } from './thread-preview-context-menu';
import { toFromNowFormattedDate } from '@/lib/utils/date';


export type ThreadPreviewComponentDto = Pick<ThreadPreviewDto, 'id' | 'title' | 'created_at' | 'assistantId'>;
export type OnThreadDelete<TThread extends ThreadPreviewComponentDto = ThreadPreviewComponentDto> = (thread: TThread) => void;
interface Props<TThread extends ThreadPreviewComponentDto = ThreadPreviewComponentDto> {
  onDelete?: OnThreadDelete<TThread>
  thread: TThread
}

export function ThreadPreview<TThread extends ThreadPreviewComponentDto = ThreadPreviewComponentDto>({ thread, onDelete }: Props<TThread>) {
  const { id, title, created_at } = thread;
  const isActive = window.location.pathname.includes(`/thread/${id}`);

  return (
    <ThreadPreviewContextMenu onDelete={() => onDelete && onDelete(thread)}>
      <Link href={`/thread/${id}`} className={cn(buttonVariants({ variant: isActive ? 'secondary' : 'outline' }), 'flex flex-col items-start select-none')}>
        <span className=''>{title}</span>
        <span className='text-xs text-slate-400'>{toFromNowFormattedDate(created_at)}</span>
        <span className='text-xs font-bold text-slate-400'>{ thread.assistantId }</span>
      </Link>
    </ThreadPreviewContextMenu>
  );
}

