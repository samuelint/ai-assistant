import { SettingsModal } from '../settings/modal';
import BrandLink from './brand.link';
import { FeedbackFormModal } from '@/components/feedback-form-modal';


export default function Header() {
  return (
    <header className="w-full bg-gray-200 text-slate-600 py-2 px-6 flex items-center">
      <div className="w-full flex items-center space-x-4 justify-between">
        <div className='flex items-center gap-4'>
          <BrandLink />
        </div>
        <div className='flex items-center gap-4'>
          <FeedbackFormModal />
          <SettingsModal />
        </div>
      </div>
    </header>
  );
}
