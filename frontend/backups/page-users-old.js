// Server Component - no 'use client' directive
import AuthLayout from '@/components/layout/AuthLayout';
import { cookies } from 'next/headers';

async function getUsers() {
  const cookieStore = await cookies();
  const token = cookieStore.get('access_token')?.value;
  
  const response = await fetch(`${process.env.NEXT_PUBLIC_API_URL}/api/users`, {
    headers: {
      'Authorization': `Bearer ${token}`,
    },
    cache: 'no-store', // Disable cache for fresh data
  });

  if (!response.ok) throw new Error('Failed to fetch users');
  
  const data = await response.json();
  return data.data || data;
}

export default async function Users() {
  const users = await getUsers(); 
  return (
    <AuthLayout>
      <div className="dashboard-container">
        <div className="container py-5">
          <h1>Users ({users.length})</h1>
          
          {/* Display users */}
          <div className="row">
            {users.map(user => (
              <div key={user.id} className="col-md-4 mb-3">
                <div className="card">
                  <div className="card-body">
                    <h5>{user.name}</h5>
                    <p className="text-muted">{user.email}</p>
                    {user.roles && (
                      <div>
                        <small>Roles: {user.roles.join(', ')}</small>
                      </div>
                    )}
                  </div>
                </div>
              </div>
            ))}
          </div>
        </div>
      </div>
    </AuthLayout>
  );
} 