import { ReactNode } from 'react';

interface LayoutProps {
  title: string;  // Title for each page
  children: ReactNode;  // Page-specific content
}

const Layout: React.FC<LayoutProps> = ({ title, children }) => {
  return (
    <div className="p-8">
      <h1 className="text-2xl font-bold mb-4">{title}</h1>
      <div>{children}</div>
    </div>
  );
};

export default Layout;
