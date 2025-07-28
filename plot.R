# library
library(ggplot2)
library(tidyr)

# Define your color palette (for continuous scale)
my_palette4 <- c("#bf616a", "#5e81ac", "#ebcb8b", "#a3be8c")
my_palette6 <- c("#bf616a", "#5e81ac", "#808080","#a3be8c","#ebcb8b","#b48ead")
my_palette7 <- c("#bf616a", "#5e81ac", "#808080","#a3be8c","#ebcb8b","#b48ead","#88c0d0")


# =========== PLOT THREADS =============
data <- read.csv("simu-out/threads.csv", header = TRUE)

# Reshape the data from wide to long format
# This is crucial for plotting multiple lines with ggplot2
data_long <- data %>%
  pivot_longer(
    cols = c(occupied, active), # Columns to pivot
    names_to = "metric",                             # New column for original column names
    values_to = "value"                              # New column for the values
  )

plot <- ggplot(data_long, aes(x = time, y = value, color = metric)) +
  geom_line(linewidth = 0.5) + # Color aesthetic is now mapped to 'metric'
  guides(color = guide_legend(position = "inside"))+
  labs(
    x = "Cycles",
    y = "Count", # Changed y-axis label to be more generic for both metrics
    title = "Threads Over Time",
  ) +
  scale_color_manual(
    breaks = c("occupied", "active"), # Original column names
    values = c("occupied" = "#5e81ac", "active" = "#bf616a"), # Custom colors for each line
    labels = c("Used resources", "Active threads") # New, custom labels for the legend
  ) +
  theme(
    plot.title = element_text(size = 12, hjust = 0.5),
    axis.title = element_text(size = 10),
    axis.text = element_text(size = 10),
    panel.grid.minor = element_blank(),
    panel.grid.major = element_line(color = "gray90"),
    panel.border = element_rect(color = "black", fill = NA, linewidth = 0.8),
    panel.background = element_rect(fill = "white"),
    axis.ticks.length = unit(-0.15, "cm"),
    
    # Legend inside plot with black border
    legend.position.inside = c(0.65, 0.3),
    legend.direction = "horizontal",  # Key change: horizontal layout
    legend.box.just = "center",      # Centers items in the legend box
    legend.text = element_text(size = 8, margin = margin(r = 1)),  # Smaller legend text
    legend.background = element_rect(
      color = "black",  # Black border
      fill = alpha("white", 0.6),   # White background
      linewidth = 0.3,   # Border thickness
    ),

    legend.margin = margin(1, 2, 1, 2),  # Tight internal padding (top,right,bottom,left)
    
    # Remove legend title
    legend.title = element_blank(),
  )

ggsave("simu-out/working-threads.pdf", width = 10, height = 5, units = "cm")

# =========== PLOT BUSY RATES =============
data1 <- read.csv("simu-out/red-rate.csv", header = TRUE)
data2 <- read.csv("simu-out/alu-rate.csv", header = TRUE)
data1$source <- "Reducer"  # Label for the first dataset
data2$source <- "ALU"  # Label for the second dataset
combined_data <- rbind(data1, data2)  # Stack them vertically

plot <- ggplot(combined_data, aes(x = time, y = rate, color = source)) +
  geom_line(linewidth = 0.5) +
  labs(x = "Cycles", y = "Busy Rate (%)", title = "Components Busy Rate Over Time") +
  scale_color_manual(values = c("Reducer" = "#5e81ac", "ALU" = "#bf616a")) +
  guides(color = guide_legend(position = "inside"))+
  theme(
    plot.title = element_text(size = 12, hjust = 0.5),  # Smaller & centered title
    axis.title = element_text(size = 10),               # Smaller axis titles
    axis.text = element_text(size = 10),               # Smaller tick labels
    
    # Remove minor gridlines (keep major if needed)
    panel.grid.minor = element_blank(),                 
    panel.grid.major = element_line(color = "gray90"),  # Lighter major grid
    
    # Black border around the plot
    panel.border = element_rect(color = "black", fill = NA, linewidth = 0.8),
    
    # Legend inside plot with black border
    legend.position.inside = c(0.7, 0.8),
    legend.direction = "horizontal",  # Key change: horizontal layout
    legend.box.just = "center",      # Centers items in the legend box
    legend.text = element_text(size = 8, margin = margin(r = 1)),  # Smaller legend text
    legend.background = element_rect(
      color = "black",  # Black border
      fill = alpha("white", 0.6),   # White background
      linewidth = 0.3,   # Border thickness
    ),

    legend.margin = margin(1, 2, 1, 2),  # Tight internal padding (top,right,bottom,left)
    
    # Remove legend title
    legend.title = element_blank(),
    
    panel.background = element_rect(fill = "white"),
    axis.ticks.length = unit(-0.15, "cm"),  # Negative value flips ticks inward
  )

ggsave("simu-out/busy-rate.pdf", width = 10, height = 6, units = "cm")

# =========== PLOT STM DISTRIBUTION =============
data <- read.csv("simu-out/stm-dist.csv", header = TRUE)
# Convert 'state' to a factor with original order
data$state <- factor(data$state, levels = data$state)

ggplot(data, aes(x = state, y = cycles)) + 
  geom_bar(stat = "identity", fill = "#5e81ac") +
  labs(title = "Bar Chart of Cycles by State", 
       x = "State", 
       y = "Cycles") +
  theme(
    plot.title = element_text(size = 12, hjust = 0.5),  # Smaller & centered title
    axis.title = element_text(size = 10),               # Smaller axis titles
    axis.text = element_text(size = 10),               # Smaller tick labels
    
    # Remove minor gridlines (keep major if needed)
    panel.grid.minor = element_blank(),                 
    panel.grid.major = element_blank(),
    
    # Black border around the plot
    panel.border = element_rect(color = "black", fill = NA, linewidth = 0.8),
    
    panel.background = element_rect(fill = "white"),
    axis.ticks.length = unit(-0.15, "cm"),  # Negative value flips ticks inward
  )
  
ggsave("simu-out/stm-dist.pdf", width = 10, height = 6, units = "cm")

# =========== PLOT BUFFER UTILISATION =============
data <- read.csv("simu-out/buffer-util.csv", header = TRUE)

data_long <- pivot_longer(data, cols = -time, names_to = "variable", values_to = "value")
ggplot(data_long, aes(x = time, y = value)) +
  geom_line(color = "#5e81ac", linewidth = 0.5)+
  facet_wrap(~ variable, ncol = 3, scales = "free_y") +
  scale_y_continuous(limits = c(0, 8)) +  # Force y-axis range
  labs(x = "Cycle", y = "Buffer Depth") +
  theme_bw()
ggsave("simu-out/buffer-util.pdf", width = 20, height = 16, units = "cm")
