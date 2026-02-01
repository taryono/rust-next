import React from "react";
import { Controller } from "react-hook-form";

export default function SelectPro({
  name,
  control,        // optional (kalau pakai RHF)
  options = [],
  value = [],
  onChange,       // optional (kalau non RHF)
  label = "Pilih",
  isMandatory = false,
  error = "",
  disabled = false,
  loading = false,
  multiple = true,

  // support dynamic key
  valueKey = "id",
  labelKey = "name",
}) {
 
// ================= SAFE NORMALIZE =================
const safeOptions = Array.isArray(options)
  ? options
  : Array.isArray(options?.data)
    ? options.data
    : [];

const normalized = safeOptions
  .filter(Boolean)
  .map(o => ({
    value: String(o?.[valueKey] ?? o?.value ?? ""),
    label: o?.[labelKey] ?? o?.label ?? "",
  }));

  const renderSelect = (field = {}) => {
    const currentValue =
      field.value ?? value ?? (multiple ? [] : "");

    const handleChange = (e) => {
      const values = multiple
        ? Array.from(e.target.selectedOptions, o => o.value)
        : e.target.value;

      field.onChange?.(values);
      onChange?.(name, values);
    };

    return (
      <div className="mb-3">
        <label className="form-label">
          {label}
          {isMandatory && (
            <span className="text-danger"> *</span>
          )}
        </label>

        <select
          className="form-select"
          multiple={multiple}
          disabled={disabled || loading}
          value={currentValue}
          onChange={handleChange}
        >
          {!multiple && (
            <option value="">
              Pilih {label}
            </option>
          )}

          {loading && (
            <option>Loading...</option>
          )}

          {!loading &&
            normalized.map((item) => (
              <option
                key={item.value}
                value={item.value}
              >
                {item.label}
              </option>
            ))}
        </select>

        {(error || field.error?.message) && (
          <div className="text-danger mt-1">
            {error || field.error?.message}
          </div>
        )}

        {/* BADGE PREVIEW */}
        {multiple && (
          <div className="mt-2 d-flex gap-2 flex-wrap">
            {normalized
              .filter(o =>
                currentValue.includes(o.value)
              )
              .map(o => (
                <span
                  key={o.value}
                  className="badge bg-primary text-white"
                >
                  {o.label}
                </span>
              ))}
          </div>
        )}
      </div>
    );
  };

  // MODE REACT HOOK FORM
  if (control) {
    return (
      <Controller
        name={name}
        control={control}
        render={({ field }) =>
          renderSelect(field)
        }
      />
    );
  }

  // MODE BIASA
  return renderSelect();
}