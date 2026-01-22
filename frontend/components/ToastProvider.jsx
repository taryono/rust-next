'use client';

import useToastStore from '@/store/toastStore';

export default function ToastProvider() {
  const { toast, clearToast } = useToastStore();

  if (!toast) return null;

  return (
    <div className="toast show position-fixed bottom-0 end-0 m-3">
      <div className="toast-body">
        {toast.message}
        <button onClick={clearToast} className="btn-close ms-2" />
      </div>
    </div>
  );
}
