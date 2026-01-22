'use client';

import { useState, useEffect } from 'react';
import { useRouter } from 'next/navigation';
import useAuthStore from '@/store/authStore';
import Link from 'next/link';
import { api } from '@/lib/api'; 
export default function LoginPage() {
    const router = useRouter();
    const { login, isLoading, error, clearError, isAuthenticated } = useAuthStore();
    const [serverStatus, setServerStatus] = useState('checking');
    const [email, setEmail] = useState('');
    const [password, setPassword] = useState('');
    const [showPassword, setShowPassword] = useState(false);

    useEffect(() => {
        if (isAuthenticated) {
            router.push('/dashboard');
        }
        const checkServer = async () => {
            try {
                await api.isOnline();
                setServerStatus('online');
            } catch (error) {
                setServerStatus('offline');
            }
        };
        setEmail('denmas.yono@gmail.com');
        setPassword('password');
        checkServer();
    }, [isAuthenticated, router]);

    const handleSubmit = async (e) => {
        e.preventDefault();
        clearError();

        const result = await login(email, password);

        if (result.success) {
            router.push('/dashboard');
        }
    };

    return (
        <>
        <div className="auth-container">
            <div className="auth-card fade-in">
                <div className="auth-header">
                    <svg xmlns="http://www.w3.org/2000/svg" width="60" height="60" fill="currentColor" className="bi bi-person-circle mb-3" viewBox="0 0 16 16">
                        <path d="M11 6a3 3 0 1 1-6 0 3 3 0 0 1 6 0z" />
                        <path fillRule="evenodd" d="M0 8a8 8 0 1 1 16 0A8 8 0 0 1 0 8zm8-7a7 7 0 0 0-5.468 11.37C3.242 11.226 4.805 10 8 10s4.757 1.225 5.468 2.37A7 7 0 0 0 8 1z" />
                    </svg>
                    <h1>Welcome Back</h1>
                    <p className="mb-0">Sign in to your account</p>
                    
                </div>

                <div className="auth-body"> 
                    {/* Server Status */}
                    <div className="mb-3">
                        {serverStatus === 'checking' && (
                            <div className="alert alert-warning">
                            <div className="d-flex">
                                <div className="spinner-border spinner-border-sm me-2"></div>
                                <div>Checking server status...</div>
                            </div>
                            </div>
                        )}
                        {serverStatus === 'online' && (
                            <div className="alert alert-success">
                            <div className="d-flex align-items-center">
                                <svg className="icon alert-icon" width="24" height="24">
                                <use xlinkHref="#tabler-check" />
                                </svg>
                                <div>Server is online</div>
                            </div>
                            </div>
                        )}
                        {serverStatus === 'offline' && (
                            <div className="alert alert-danger">
                            <div className="d-flex align-items-center">
                                <svg className="icon alert-icon" width="24" height="24">
                                <use xlinkHref="#tabler-alert-triangle" />
                                </svg>
                                <div>Server is offline</div>
                            </div>
                            </div>
                        )}
                    </div>
                    {error && (
                        <div className="alert alert-danger alert-dismissible fade show" role="alert">
                            <i className="bi bi-exclamation-triangle-fill me-2"></i>
                            {error}
                            <button type="button" className="btn-close" onClick={clearError}></button>
                        </div>
                    )}

                    <form onSubmit={handleSubmit}>
                        <div className="mb-3">
                            <label htmlFor="email" className="form-label fw-semibold">
                                <i className="bi bi-envelope me-2"></i>Email Address
                            </label>
                            <input
                                type="email"
                                className="form-control form-control-lg"
                                id="email"
                                placeholder="Enter your email"
                                value={email}
                                onChange={(e) => setEmail(e.target.value)}
                                required
                                disabled={isLoading}
                            />
                        </div>

                        <div className="mb-3">
                            <label htmlFor="password" className="form-label fw-semibold">
                                <i className="bi bi-lock me-2"></i>Password
                            </label>
                            <div className="input-group">
                                <input
                                    type={showPassword ? 'text' : 'password'}
                                    className="form-control form-control-lg"
                                    id="password"
                                    placeholder="Enter your password"
                                    value={password}
                                    onChange={(e) => setPassword(e.target.value)}
                                    required
                                    disabled={isLoading}
                                />
                                <button
                                    className="btn btn-outline-secondary"
                                    type="button"
                                    onClick={() => setShowPassword(!showPassword)}
                                    disabled={isLoading}
                                >
                                    <i className={`bi bi-eye${showPassword ? '-slash' : ''}`}></i>
                                </button>
                            </div>
                        </div>

                        <div className="mb-3 form-check">
                            <input type="checkbox" className="form-check-input" id="rememberMe" />
                            <label className="form-check-label" htmlFor="rememberMe">
                                Remember me
                            </label>
                        </div>

                        <button
                            type="submit"
                            className="btn btn-primary btn-lg w-100 mb-3"
                            disabled={isLoading || serverStatus !== 'online'}
                        >
                            {isLoading ? (
                                <>
                                    <span className="spinner-border spinner-border-sm me-2" role="status" aria-hidden="true"></span>
                                    Signing in...
                                </>
                            ) : (
                                <>
                                    <i className="bi bi-box-arrow-in-right me-2"></i>
                                    Sign In
                                </>
                            )}
                        </button>
                    </form>

                    <div className="text-center">
                        <p className="text-muted mb-0">
                            Don&apos;t have an account?{' '}
                            <Link href="/register" className="text-decoration-none fw-semibold">
                                Create Account
                            </Link>
                        </p>
                    </div>
                </div>
            </div>
        </div> 
        </>
    );
}