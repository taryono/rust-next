import React from "react";
import { Controller } from "react-hook-form";
import Select from "react-select";

export default function SelectPro({
  name,
  control,
  options = [],
  value = [],
  onChange,
  label = "Pilih",
  isMandatory = false,
  error = "",
  disabled = false,
  loading = false,
  multiple = true,
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
    .map((o) => ({
      value: String(o?.[valueKey] ?? o?.value ?? ""),
      label: o?.[labelKey] ?? o?.label ?? "",
    }));

  // ======= Konversi value saat ini ke format react-select =======
  const getSelectedOptions = (currentValue) => {
    if (multiple) {
      const vals = Array.isArray(currentValue) ? currentValue : [];
      return normalized.filter((o) => vals.includes(o.value));
    }
    return normalized.find((o) => o.value === currentValue) || null;
  };

  const renderSelect = (field = {}) => {
    const currentValue = field.value ?? value ?? (multiple ? [] : "");

    const handleChange = (selected) => {
      // selected = array kalau multiple, single object kalau single
      const newValue = multiple
        ? (selected || []).map((s) => s.value)
        : selected?.value ?? "";

      field.onChange?.(newValue);
      onChange?.(name, newValue);
    };

    return (
      <div className="mb-3"> 
        <Select
          isMulti={multiple}
          options={normalized}
          value={getSelectedOptions(currentValue)}
          onChange={handleChange}
          isDisabled={disabled || loading}
          isLoading={loading}
          placeholder={`Pilih ${label}...`}
          noOptionsMessage={() => "Tidak ada pilihan"}
          loadingMessage={() => "Loading..."}
          // Styling agar sesuai Bootstrap
          styles={{
            control: (base, state) => ({
              ...base,
              borderColor: error || field.error?.message
                ? "#dc3545"
                : state.isFocused ? "#0d6efd" : "#ced4da",
              borderRadius: "0.375rem",
              minHeight: "38px",
              boxShadow: state.isFocused
                ? "0 0 0 0.2rem rgba(13,110,253,0.25)"
                : "none",
            }),
            option: (base, state) => ({
              ...base,
              backgroundColor: state.isSelected
                ? "#0d6efd"
                : state.isFocused ? "#e7f1ff" : "white",
              color: state.isSelected ? "white" : "#333",
            }),
            multiValue: (base) => ({
              ...base,
              backgroundColor: "#0d6efd",
              borderRadius: "0.25rem",
            }),
            multiValueLabel: (base) => ({
              ...base,
              color: "white",
            }),
            multiValueRemove: (base) => ({
              ...base,
              color: "white",
              ":hover": { backgroundColor: "#0a58ca", color: "white" },
            }),
          }}
        />

        {(error || field.error?.message) && (
          <div className="text-danger mt-1 small">
            {error || field.error?.message}
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
        render={({ field }) => renderSelect(field)}
      />
    );
  }

  // MODE BIASA
  return renderSelect();
}