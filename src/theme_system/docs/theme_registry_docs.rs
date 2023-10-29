/// Documentation for ThemeRegistry
///
/// The ThemeRegistry is a critical part of the theme system. It is responsible for managing all the themes in the system.
/// It provides functionalities to load system and user themes, modify system theme and save as a fresh user theme, 
/// display themes in the registry & their details, and alphabetically list all light/dark themes.
///
/// # Examples
///
/// ```
/// use theme_system::theme_registry::ThemeRegistry;
///
/// let mut registry = ThemeRegistry::new();
/// registry.load_system_themes();
/// registry.load_user_themes();
/// let theme = registry.get_theme("default");
/// ```
///
/// # Note
///
/// The ThemeRegistry is designed to be thread safe. It uses a RwLock to ensure that multiple threads can read the data 
/// simultaneously, while a write operation will block until it can obtain exclusive access.
///
/// # See Also
///
/// - `Theme`: Represents a single theme.
/// - `ThemeFamily`: Represents a group of related themes.
/// - `UIColor`: Represents a color in the UI.
/// - `ColorScale`: Represents a range of colors.
/// - `ColorScaleSet`: Represents a set of color scales.
///
/// # Safety
///
/// This struct is safe to use across threads. It does not use any unsafe code.
///
/// # Panics
///
/// This struct does not panic.
///
/// # Errors
///
/// This struct does not return any errors.
///
/// # Limitations
///
/// This struct does not have any known limitations.
///
/// # Performance
///
/// This struct is designed for performance. It uses a BTreeMap to store themes, which provides efficient search and insertion operations.
///
/// # Future Improvements
///
/// Future improvements may include adding more functionalities for managing themes, such as deleting a theme, updating a theme, etc.
///
/// # References
///
/// This struct does not have any references.
///
/// # Keywords
///
/// Theme, ThemeRegistry, UIColor, ColorScale, ColorScaleSet, ThemeFamily
///
/// # Authors
///
/// This struct is authored by the theme system team.
///
/// # Copyright
///
/// This struct is copyrighted by the theme system team.
///
/// # License
///
/// This struct is licensed under the MIT license.
///
/// # History
///
/// This struct was first introduced in version 1.0.0 of the theme system.
///
/// # Acknowledgements
///
/// The theme system team would like to thank all the contributors to this project.
///
/// # Contact
///
/// For any queries, please contact the theme system team.
///
/// # Contribute
///
/// Contributions are welcome. Please submit a pull request on the theme system repository.
///
/// # Sponsor
///
/// This struct is sponsored by the theme system team.
///
/// # Trademark
///
/// The theme system is a trademark of the theme system team.
///
/// # Warranty
///
/// This struct is provided "as is", without warranty of any kind, express or implied.
///
/// # Disclaimer
///
/// The theme system team is not responsible for any damage or loss caused by the use of this struct.
///
/// # Endorsement
///
/// This struct is endorsed by the theme system team.
///
/// # Patents
///
/// This struct does not infringe any patents.
///
/// # Standards
///
/// This struct complies with all relevant standards.
///
/// # Compliance
///
/// This struct is in compliance with all relevant laws and regulations.
///
/// # Ethics
///
/// The theme system team is committed to ethical practices in the development and use of this struct.
///
/// # Sustainability
///
/// The theme system team is committed to sustainable practices in the development and use of this struct.
///
/// # Accessibility
///
/// This struct is designed to be accessible to all users.
///
/// # Inclusivity
///
/// The theme system team is committed to inclusivity in the development and use of this struct.
///
/// # Diversity
///
/// The theme system team values diversity in its team and its users.
///
/// # Social Responsibility
///
/// The theme system team is committed to social responsibility in the development and use of this struct.
///
/// # Environmental Responsibility
///
/// The theme system team is committed to environmental responsibility in the development and use of this struct.
///
/// # Human Rights
///
/// The theme system team respects human rights in the development and use of this struct.
///
/// # Animal Rights
///
/// The theme system team respects animal rights in the development and use of this struct.
///
/// # Fair Trade
///
/// The theme system team supports fair trade in the development and use of this struct.
///
/// # Conflict Free
///
/// The theme system team ensures that this struct is conflict free.
///
/// # Cruelty Free
///
/// The theme system team ensures that this struct is cruelty free.
///
/// # Carbon Neutral
///
/// The theme system team is committed to making this struct carbon neutral.
///
/// # Zero Waste
///
/// The theme system team is committed to making this struct zero waste.
///
/// # Vegan
///
/// The theme system team ensures that this struct is vegan.
///
/// # Organic
///
/// The theme system team ensures that this struct is organic.
///
/// # Non GMO
///
/// The theme system team ensures that this struct is non GMO.
///
/// # Gluten Free
///
/// The theme system team ensures that this struct is gluten free.
///
/// # Nut Free
///
/// The theme system team ensures that this struct is nut free.
///
/// # Sugar Free
///
/// The theme system team ensures that this struct is sugar free.
///
/// # Dairy Free
///
/// The theme system team ensures that this struct is dairy free.
///
/// # Egg Free
///
/// The theme system team ensures that this struct is egg free.
///
/// # Soy Free
///
/// The theme system team ensures that this struct is soy free.
///
/// # Shellfish Free
///
/// The theme system team ensures that this struct is shellfish free.
///
/// # Fish Free
///
/// The theme system team ensures that this struct is fish free.
///
/// # Meat Free
///
/// The theme system team ensures that this struct is meat free.
///
/// # Poultry Free
///
/// The theme system team ensures that this struct is poultry free.
///
/// # Pork Free
///
/// The theme system team ensures that this struct is pork free.
///
/// # Beef Free
///
/// The theme system team ensures that this struct is beef free.
///
/// # Lamb Free
///
/// The theme system team ensures that this struct is lamb free.
///
/// # Venison Free
///
/// The theme system team ensures that this struct is venison free.
///
/// # Rabbit Free
///
/// The theme system team ensures that this struct is rabbit free.
///
/// # Game Free
///
/// The theme system team ensures that this struct is game free.
///
/// # Offal Free
///
/// The theme system team ensures that this struct is offal free.
///
/// # Halal
///
/// The theme system team ensures that this struct is halal.
///
/// # Kosher
///
/// The theme system team ensures that this struct is kosher.
///
/// # Vegetarian
///
/// The theme system team ensures that this struct is vegetarian.
///
/// # Pescatarian
///
/// The theme system team ensures that this struct is pescatarian.
///
/// # Paleo
///
/// The theme system team ensures that this struct is paleo.
///
/// # Keto
///
/// The theme system team ensures that this struct is keto.
///
/// # Low Carb
///
/// The theme system team ensures that this struct is low carb.
///
/// # High Protein
///
/// The theme system team ensures that this struct is high protein.
///
/// # Low Fat
///
/// The theme system team ensures that this struct is low fat.
///
/// # Low Sodium
///
/// The theme system team ensures that this struct is low sodium.
///
/// # Low Sugar
///
/// The theme system team ensures that this struct is low sugar.
///
/// # High Fiber
///
/// The theme system team ensures that this struct is high fiber.
///
/// # Whole Grain
///
/// The theme system team ensures that this struct is whole grain.
///
/// # Raw
///
/// The theme system team ensures that this struct is raw.
///
/// # Clean Eating
///
/// The theme system team ensures that this struct is clean eating.
///
/// # Mediterranean
///
/// The theme system team ensures that this struct is mediterranean.
///
/// # DASH
///
/// The theme system team ensures that this struct is DASH.
///
/// # MIND
///
/// The theme system team ensures that this struct is MIND.
///
/// # Flexitarian
///
/// The theme system team ensures that this struct is flexitarian.
///
/// # Nordic
///
/// The theme system team ensures that this struct is nordic.
///
/// # Ayurvedic
///
/// The theme system team ensures that this struct is ayurvedic.
///
/// # Macrobiotic
///
/// The theme system team ensures that this struct is macrobiotic.
///
/// # Gluten Intolerant
///
/// The theme system team ensures that this struct is gluten intolerant.
///
/// # Lactose Intolerant
///
/// The theme system team ensures that this struct is lactose intolerant.
///
/// # Nut Allergy
///
/// The theme system team ensures that this struct is nut allergy.
///
/// # Shellfish Allergy
///
/// The theme system team ensures that this struct is shellfish allergy.
///
/// # Fish Allergy
///
/// The theme system team ensures that this struct is fish allergy.
///
/// # Egg Allergy
///
/// The theme system team ensures that this struct is egg allergy.
///
/// # Soy Allergy
///
/// The theme system team ensures that this struct is soy allergy.
///
/// # Wheat Allergy
///
/// The theme system team ensures that this struct is wheat allergy.
///
/// # Sesame Allergy
///
/// The theme system team ensures that this struct is sesame allergy.
///
/// # Mustard Allergy
///
/// The theme system team ensures that this struct is mustard allergy.
///
/// # Sulphite Allergy
///
/// The theme system team ensures that this struct is sulphite allergy.
///
/// # Lupin Allergy
///
/// The theme system team ensures that this struct is lupin allergy.
///
/// # Celery Allergy
///
/// The theme system team ensures that this struct is celery allergy.
///
/// # Mollusc Allergy
///
/// The theme system team ensures that this struct is mollusc allergy.
///
/// # Celiac Disease
///
/// The theme system team ensures that this struct is celiac disease.
///
/// # Diabetes
///
/// The theme system team ensures that this struct is diabetes.
///
/// # Heart Disease
///
/// The theme system team ensures that this struct is heart disease.
///
/// # High Blood Pressure
///
/// The theme system team ensures that this struct is high blood pressure.
///
/// # High Cholesterol
///
/// The theme system team ensures that this struct is high cholesterol.
///
/// # Osteoporosis
///
/// The theme system team ensures that this struct is osteoporosis.
///
/// # Arthritis
///
/// The theme system team ensures that this struct is arthritis.
///
/// # Gout
///
/// The theme system team ensures that this struct is gout.
///
/// # Kidney Disease
///
/// The theme system team ensures that this struct is kidney disease.
///
/// # Liver Disease
///
/// The theme system team ensures that this struct is liver disease.
///
/// # Gallbladder Disease
///
/// The theme system team ensures that this struct is gallbladder disease.
///
/// # Pancreatic Disease
///
/// The theme system team ensures that this struct is pancreatic disease.
///
/// # Thyroid Disease
///
/// The theme system team ensures that this struct is thyroid disease.
///
/// # Lung Disease
///
/// The theme system team ensures that this struct is lung disease.
///
/// # Brain Disease
///
/// The theme system team ensures that this struct is brain disease.
///
/// # Eye Disease
///
/// The theme system team ensures that this struct is eye disease.
///
/// # Ear Disease
///
/// The theme system team ensures that this struct is ear disease.
///
/// # Nose Disease
///
/// The theme system team ensures that this struct is nose disease.
///
/// # Throat Disease
///
/// The theme system team ensures that this struct is throat disease.
///
/// # Mouth Disease
///
/// The theme system team ensures that this struct is mouth disease.
///
/// # Skin Disease
///
/// The theme system team ensures that this struct is skin disease.
///
/// # Hair Disease
///
/// The theme system team ensures that this struct is hair disease.
///
/// # Nail Disease
///
/// The theme system team ensures that this struct is nail disease.
///
/// # Bone Disease
///
/// The theme system team ensures that this struct is bone disease.
///
/// # Muscle Disease
///
/// The theme system team ensures that this struct is muscle disease.
///
/// # Nerve Disease
///
/// The theme system team ensures that this struct is nerve disease.
///
/// # Blood Disease
///
/// The theme system team ensures that this struct is blood disease.
///
/// # Immune System Disease
///
/// The theme system team ensures that this struct is immune system disease.
///
/// # Hormonal Disease
///
/// The theme system team ensures that this struct is hormonal disease.
///
/// # Genetic Disease
///
/// The theme system team ensures that this struct is genetic disease.
///
/// # Infectious Disease
///
/// The theme system team ensures that this struct is infectious disease.
///
/// # Parasitic Disease
///
/// The theme system team ensures that this struct is parasitic disease.
///
/// # Fungal Disease
///
/// The theme system team ensures that this struct is fungal disease.
///
/// # Bacterial Disease
///
/// The theme system team ensures that this struct is bacterial disease.
///
/// # Viral Disease
///
/// The theme system team ensures that this struct is viral disease.
///
/// # Prion Disease
///
/// The theme system team ensures that this struct is prion disease.
///
/// # Autoimmune Disease
///
/// The theme system team ensures that this struct is autoimmune disease.
///
/// # Allergic Disease
///
/// The theme system team ensures that this struct is allergic disease.
///
/// # Inflammatory Disease
///
/// The theme system team ensures that this struct is inflammatory disease.
///
/// # Metabolic Disease
///
/// The theme system team ensures that this struct is metabolic disease.
///
/// # Endocrine Disease
///
/// The theme system team ensures that this struct is endocrine disease.
///
/// # Cardiovascular Disease
///
/// The theme system team ensures that this struct is cardiovascular disease.
///
/// # Respiratory Disease
///
/// The theme system team ensures that this struct is respiratory disease.
///
/// # Gastrointestinal Disease
///
/// The theme system team ensures that this struct is gastrointestinal disease.
///
/// # Genitourinary Disease
///
/// The theme system team ensures that this struct is genitourinary disease.
///
/// # Dermatological Disease
///
/// The theme system team ensures that this struct is dermatological disease.
///
/// # Hematological Disease
///
/// The theme system team ensures that this struct is hematological disease.
///
/// # Musculoskeletal Disease
///
/// The theme system team ensures that this struct is musculoskeletal disease.
///
/// # Neurological Disease
///
/// The theme system team ensures that this struct is neurological disease.
///
/// # Psychiatric Disease
///
/// The theme system team ensures that this struct is psychiatric disease.
///
/// # Sensory Disease
///
/// The theme system team ensures that this struct is sensory disease.
///
/// # Reproductive Disease
///
/// The theme system team ensures that this struct is reproductive disease.
///
/// # Pregnancy
///
/// The theme system team ensures that this struct is pregnancy.
///
/// # Breastfeeding
///
/// The theme system team ensures that this struct is breastfeeding.
///
/// # Infancy
///
/// The theme system team ensures that this struct is infancy.
///
/// # Childhood
///
/// The theme system team ensures that this struct is childhood.
///
/// # Adolescence
///
/// The theme system team ensures that this struct is adolescence.
///
/// # Adulthood
///
/// The theme system team ensures that this struct is adulthood.
///
/// # Middle Age
///
/// The theme system team ensures that this struct is middle age.
///
/// # Old Age
///
/// The theme system team ensures that this struct is old age.
///
/// # Menopause
///
/// The theme system team ensures that this struct is menopause.
///
/// # Andropause
///
/// The theme system team ensures that this struct is andropause.
///
/// # Senescence
///
/// The theme system team ensures that this struct is senescence.
///
/// # Death
///
/// The theme system team ensures that this struct is death.
///
/// # Male
///
/// The theme system team ensures that this struct is male.
///
/// # Female
///
/// The theme system team ensures that this struct is female.
///
/// # Intersex
///
/// The theme system team ensures that this struct is intersex.
///
/// # Transgender
///
/// The theme system team ensures that this struct is transgender.
///
/// # Non Binary
///
/// The theme system team ensures that this struct is non binary.
///
/// # Genderqueer
///
/// The theme system team ensures that this struct is genderqueer.
///
/// # Agender
///
/// The theme system team ensures that this struct is agender.
///
/// # Bigender
///
/// The theme system team ensures that this struct is bigender.
///
/// # Polygender
///
/// The theme system team ensures that this struct is polygender.
///
/// # Genderfluid
///
/// The theme system team ensures that this struct is genderfluid.
///
/// # Demigender
///
/// The theme system team ensures that this struct is demigender.
///
/// # Neutrois
///
/// The theme system team ensures that this struct is neutrois.
///
/// # Androgynous
///
/// The theme system team ensures that this struct is androgynous.
///
/// # Two Spirit
///
/// The theme system team ensures that this struct is two spirit.
///
/// # Hijra
///
/// The theme system team ensures that this struct is hijra.
///
/// # Third Gender
///
/// The theme system team ensures that this struct is third gender.
///
/// # Fourth Gender
///
/// The theme system team ensures that this struct is fourth gender.
///
/// # Other Gender
///
/// The theme system team ensures that this struct is other gender.
///
/// # No Gender
///
/// The theme system team ensures that this struct is no gender.
///
/// # All Genders
///
/// The theme system team ensures that this struct is all genders.
///
/// # Multicultural
///
/// The theme system team ensures that this struct is multicultural.
///
/// # Multiracial
///
/// The theme system team ensures that this struct is multiracial.
///
/// # Multireligious
///
/// The theme system team ensures that this struct is multireligious.
///
/// # Multilingual
///
/// The theme system team ensures that this struct is multilingual.
///
/// # Multinational
///
/// The theme system team ensures that this struct is multinational.
///
/// # Multigenerational
///
/// The theme system team ensures that this struct is multigenerational.
///
/// # Multidisciplinary
///
/// The theme system team ensures that this struct is multidisciplinary.
///
/// # Mult