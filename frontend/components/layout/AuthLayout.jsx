import Navbar from './Navbar';
import Sidebar from './Sidebar';

export default function AuthLayout({ children }) {
  return (
    <>
      <Navbar />
      <div className="d-flex">
        <Sidebar />
        <main className="flex-fill p-4">{children}</main>
      </div>
    </>
  );
}
