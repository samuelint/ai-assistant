import './globals.css';
import { Toaster } from '@/components/ui/toaster';
import Header from '@/app/_components/header';
import { Inter } from 'next/font/google';


export const inter = Inter({ subsets: ['latin'] });

export function MainLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <>
      <div className='fixed w-full'>
        <Header />
      </div>
      <div className='w-full h-full pt-10 overflow-hidden'>
        {children}
      </div>
      <Toaster />
    </>
  );
}
