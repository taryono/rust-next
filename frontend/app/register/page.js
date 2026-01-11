'use client';

import { useState, useEffect } from 'react';
import { useRouter } from 'next/navigation';
import useAuthStore from '@/store/authStore';
import Link from 'next/link';

export default function RegisterPage() {
    const router = useRouter();
    const { register, isLoading, error, clearError, isAuthenticated } = useAuthStore();

    const [name, setName] = useState('');
    const [email, setEmail] = useState('');
    const [password, setPassword] = useState('');
    const [confirmPassword, setConfirmPassword] = useState('');
    const [showPassword, setShowPassword] = useState(false);
    const [validationError, setValidationError] = useState('');

    useEffect(() => {
        if (isAuthenticated) {
            router.push('/dashboard');
        }
    }, [isAuthenticated, router]);

    const handleSubmit = async (e) => {
        e.preventDefault();
        clearError();
        setValidationError('');

        // Validation
        if (name.length < 3) {
            setValidationError('Name must be at least 3 characters');
            return;
        }

        if (password.length < 6) {
            setValidationError('Password must be at least 6 characters');
            return;
        }

        if (password !== confirmPassword) {
            setValidationError('Passwords do not match');
            return;
        }

        const result = await register(name, email, password);

        if (result.success) {
            router.push('/dashboard');
        }
    };

    return (
        <div className="auth-container">
            <div className="auth-card fade-in">
                <div className="auth-header">
                    <svg xmlns="http://www.w3.org/2000/svg" width="60" height="60" fill="currentColor" className="bi bi-person-plus mb-3" viewBox="0 0 16 16">
                        <path d="M6 8a3 3 0 1 0 0-6 3 3 0 0 0 0 6zm2-3a2 2 0 1 1-4 0 2 2 0 0 1 4 0zm4 8c0 1-1 1-1 1H1s-1 0-1-1 1-4 6-4 6 3 6 4zm-1-.004c-.001-.246-.154-.986-.832-1.664C9.516 10.68 8.289 10 6 10c-2.29 0-3.516.68-4.168 1.332-.678.678-.83 1.418-.832 1.664h10z" />
                        <path fillRule="evenodd" d="M13.5 5a.5.5 0 0 1 .5.5V7h1.5a.5.5 0 0 1 0 1H14v1.5a.5.5 0 0 1-1 0V8h-1.5a.5.5 0 0 1 0-1H13V5.5a.5.5 0 0 1 .5-.5z" />
                    </svg>
                    <h1>Create Account</h1>
                    <p className="mb-0">Join us today</p>
                </div>

                <div className="auth-body">
                    {(error || validationError) && (
                        <div className="alert alert-danger alert-dismissible fade show" role="alert">
                            <i className="bi bi-exclamation-triangle-fill me-2"></i>
                            {error || validationError}
                            <button type="button" className="btn-close" onClick={() => { clearError(); setValidationError(''); }}></button>
                        </div>
                    )}

                    <form onSubmit={handleSubmit}>
                        <div className="mb-3">
                            <label htmlFor="name" className="form-label fw-semibold">
                                <i className="bi bi-person me-2"></i>Full Name
                            </label>
                            <input
                                type="text"
                                className="form-control form-control-lg"
                                id="name"
                                placeholder="Enter your full name"
                                value={name}
                                onChange={(e) => setName(e.target.value)}
                                required
                                disabled={isLoading}
                                minLength={3}
                            />
                            <small className="text-muted">At least 3 characters</small>
                        </div>

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
                                    placeholder="Create password"
                                    value={password}
                                    onChange={(e) => setPassword(e.target.value)}
                                    required
                                    disabled={isLoading}
                                    minLength={6}
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
                            <small className="text-muted">At least 6 characters</small>
                        </div>

                        <div className="mb-3">
                            <label htmlFor="confirmPassword" className="form-label fw-semibold">
                                <i className="bi bi-lock-fill me-2"></i>Confirm Password
                            </label>
                            <input
                                type={showPassword ? 'text' : 'password'}
                                className="form-control form-control-lg"
                                id="confirmPassword"
                                placeholder="Confirm your password"
                                value={confirmPassword}
                                onChange={(e) => setConfirmPassword(e.target.value)}
                                required
                                disabled={isLoading}
                            />
                        </div>

                        <button
                            type="submit"
                            className="btn btn-primary btn-lg w-100 mb-3"
                            disabled={isLoading}
                        >
                            {isLoading ? (
                                <>
                                    <span className="spinner-border spinner-border-sm me-2" role="status" aria-hidden="true"></span>
                                    Creating account...
                                </>
                            ) : (
                                <>
                                    <i className="bi bi-person-check me-2"></i>
                                    Create Account
                                </>
                            )}
                        </button>
                    </form>

                    <div className="text-center">
                        <p className="text-muted mb-0">
                            Already have an account?{' '}
                            <Link href="/login" className="text-decoration-none fw-semibold">
                                Sign In
                            </Link>
                        </p>
                    </div>
                </div>
            </div>
        </div>
    );
}