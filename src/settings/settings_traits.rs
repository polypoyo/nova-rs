/*
 * \brief Anything which inherits from this class wants to know about the configuration and any changes to it
 */
use crate::settings::settings_structs::*;

pub trait ConfigListener {
    /*
     * \brief Tells the listeners that there has been a change in the configuration
     *
     * This method is called throughout Nova's lifetime whenever a configuration value changes. This method should
     * handle changing configuration values such as the size of the window and what shaderpack the user has loaded
     *
     * Note that this method only receives the read-write config values (the 'settings' node)
     *
     * \param new_config The updated configuration
     */
    fn on_config_change(&self, new_config: &NovaSettings);

    /*
     * \brief Tells listeners that the configuration has been loaded
     *
     * When Nova starts up, this method is called on all config listeners, then on_config_change is called.
     * on_config_change should be used to listen for any config values that change throughout the program's life, so
     * then this method should be used for any initial configuration whose values will not change throughout the
     * program's lifetime. An example of this is reading in the bind points of the UBOs: the bind points won't change
     * throughout the program's life, so they should be handled in this function
     *
     * We may want to consider two config files: one for read-only values and one for read-write values. Probably a
     * good idea, but I don't feel like implementing that just yet
     *
     * \param config The configuration that was loaded
     */
    fn on_config_loaded(&self, config: &NovaSettings);
}
