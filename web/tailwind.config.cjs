module.exports = {
  future: {
    purgeLayersByDefault: true,
    removeDeprecatedGapUtilities: true,
  },
  plugins: [],
  purge: {
    content: ["./**/*.html", "./src/**/*.svelte"],
    enabled: false,
    // enabled: production,
  },
};
